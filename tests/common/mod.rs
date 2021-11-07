use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::path;

/*
   OS eastings & northings (eastings always precede northings) can be expressed either
   as pairs of digits, or pairs of digits preceded by a two-character grid ref.
   E.g.: 485669,092167 or SZ 85669 92167.

   See: https://en.wikipedia.org/wiki/Ordnance_Survey_National_Grid for grid details.

   A grid ref reduces the number coords required. Where the grid ref is omitted,
   the numeric values must have their origin at the south west corner of the full GB grid.

   Alphanumeric coordinates
    • may be separated by spaces but not always, e.g. SZ8554592142
    • should have the same number of digits in the eastings & northings

   Alphanumeric coordinates may be shortened to reduce accuracy:
   SZ 85545 92142   10 digit pair accurate to    1 m
   SZ 8554 9214      8 digit pair accurate to   10 m
   SZ 855 921        6 digit pair accurate to  100 m
   SZ 85 92          4 digit pair accurate to    1 km
   SX 8 9            2 digit pair accurate to   10 km
   SZ                0 digit pair accurate to  100 km

   Numeric coordinates
    • may be separated by a space, a comma, or both
    • pairs usually contain the same number of digits but may contain different
      numbers of digits in Orkney & Shetland where northings extend beyond 999999
      to require 7 digits, e.g. Ronas Hill (Shetland): 430530,1183500 (HU 30530 83500)
*/

// British National Grid
const GRIDS_PER_ROW_100: i64 = 7;         // No. of grids per row in the full 91 grid block
const METRES_IN_500_GRID: i64 = 500_000;  // No of metres in 500 Km² grid E & N
const METRES_IN_100_GRID: i64 = 100_000;  // No of metres in 100 Km² grid E & N
const METRES_IN_10_GRID: i64 = 10_000;    // No of metres in 10 Km² grid E & N
const DATA_COLS_IN_10_GRID: i64 = 10;     // No. of columns in a 10km² grid

// Data file sig
const FILE_SIG: &[u8; 11] = b"OSTerrain50";

// Data file header section
const MAX_NUM_DATA_FILES: i64 = 100;      // Maximum number of data files per 10km² grid
const GRID_IDENT_LEN: i64 = 2;            // Length of a grid identifer ("SV" etc.)
const ADDRESS_LENGTH: i64 = 4;            // Length of data addresses stored in the output file
const HEADER_BLOCK_LENGTH: i64 = GRID_IDENT_LEN + (MAX_NUM_DATA_FILES * ADDRESS_LENGTH);

// Data file data section
const ELEVATIONS_PER_COL: i64 = 200;      // No. of elevation values in each data column
const ELEVATION_DATA_LENGTH: i64 = 2;     // Length of a single elvation data point
const ELEVATION_DISTANCE: i64 = 50;       // Distance between successive elevations points

#[derive(Debug, Clone, Copy)]
pub struct OSCoords {
    pub easting: i64,
    pub northing: i64,
    pub elevation: Option<f32>,
}

pub fn read_elevations(
    data_file: &str,
    coords_list: &[OSCoords],
    infill: bool,
) -> Result<Vec<OSCoords>, Box<dyn Error>> {

    // Returns a vec of the supplied coordinates with the elevation provided for each coordinate.
    // Optionally creates infill coordinates and elevations at approx. 50m intervals between
    // each coordinate pair.

    if coords_list.len() < 1 {
        panic!("Need at least one location in the coords list");
    }

    let mut coords: Vec<OSCoords> = Vec::new();

    if infill {
        for i in 0..coords_list.len() {
            // Need at least two locations to prepare infills
            if i > 0 {
                let mut include_start = true;
                if i >= 2 {
                    // Avoid double insertions where previous end == current start
                    include_start = false;
                }
                let infills = get_infills(coords_list[i - 1], coords_list[i], include_start);
                // Merge the results
                coords.extend(infills);               
            }
        }
    } else {
        // No infills required so just process the input locations
        coords = coords_list.to_vec();
    }

    let file_path = path::Path::new(&data_file);
    if !file_path.is_file() {
        panic!("The data file path {} is not valid", &data_file);
    }
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    for i in 0..coords.len() {

        // Work out how many grid blocks to jump over in the file header section.
        // NB: uses integer division to deliberately truncate the remainders - use floor(),
        // trunc() etc. in untyped languages

        // Reduce the coords down to obtain whole grid unit multipliers and apply them to
        // calculate the mumber of grids to jump over
        let e_cols = coords[i].easting / METRES_IN_100_GRID;
        let n_rows = coords[i].northing / METRES_IN_100_GRID;
        let grid_blocks_to_jump = (GRIDS_PER_ROW_100 * n_rows) + e_cols;

        // Calculate the offset from start of file to the start of the required grid block
        let grid_block_offset = FILE_SIG.len() as i64 + (grid_blocks_to_jump * HEADER_BLOCK_LENGTH);

        // Now work out how many data address placeholders to jump within the grid block section.
        // (NB: integer division truncation)
        let e_addr_cols = (coords[i].easting % METRES_IN_100_GRID) / METRES_IN_10_GRID;
        let n_addr_rows = (coords[i].northing % METRES_IN_100_GRID) / METRES_IN_10_GRID;
        let data_placeholders_to_jump = (n_addr_rows * DATA_COLS_IN_10_GRID) + e_addr_cols;

        // Can now determine the individual data block address to jump to within the grid block
        let data_block_address_offset =
            (grid_block_offset + GRID_IDENT_LEN + (data_placeholders_to_jump * ADDRESS_LENGTH))
                as u64;

        // Finally, work out the offset required to get to the desired elevation within a data block.
        // First reduce the coords to just the parts applicable in a 10 Km² data grid
        let data_easting = (coords[i].easting % METRES_IN_100_GRID) % METRES_IN_10_GRID;
        let data_northing = (coords[i].northing % METRES_IN_100_GRID) % METRES_IN_10_GRID;

        // Then work out how many data rows and columns must be jumped (elevations are every 50m)
        // (NB: integer division truncation)
        let data_cols = data_easting / ELEVATION_DISTANCE;
        let data_rows = data_northing / ELEVATION_DISTANCE;
        let data_rows_to_jump = (data_rows * ELEVATIONS_PER_COL) + data_cols;

        let elevation_offset = (data_rows_to_jump * ELEVATION_DATA_LENGTH) as u64;

        // Jump to the location of the data block address
        reader.seek(SeekFrom::Start(data_block_address_offset as u64))?;

        // Read the four byte data address value stored there into a buffer
        let mut address_buffer = [0; ADDRESS_LENGTH as usize];
        reader.read_exact(&mut address_buffer)?;

        // If there is a non-zero stored value there, convert it to
        // a little endian address value
        let data_block_address = u32::from_le_bytes(address_buffer) as u64;
        if data_block_address != 0 {

            // Apply the required elevation data offset to the data block address
            // and jump there
            let elev_addr = data_block_address + elevation_offset;
            reader.seek(SeekFrom::Start(elev_addr))?;

            // Read the elevation data as two bytes
            let mut elevation_buffer = [0; ELEVATION_DATA_LENGTH as usize];
            reader.read_exact(&mut elevation_buffer)?;

            // Because elevation data never has more than one decimal place, it's stored
            // as 10x actual value as little endian i16 for space-efficient storage
            let elev_x10 = i16::from_le_bytes(elevation_buffer);
            coords[i].elevation = Some(elev_x10 as f32 / 10f32);
        }
        else {
            // No data address means no data exists for this location, i.e. it's a 
            // sea area or an out-of-scope land mass, e.g. the Isle of Man
            coords[i].elevation = Some(0 as f32);
        }
    }
    Ok(coords)
}

fn get_infills(coord_start: OSCoords, coord_end: OSCoords, include_start: bool) -> Vec<OSCoords> {

    /*
       Creates infill locations approx. 50m apart between the two parameter locations

       The include_start parameter controls whether the start is included in the output
       in order to avoid double insertions when later merging infilled locations
       
       Example: for 4 locations requiring infills:
       1---2               get_infills(1, 2, true)  returns 1st, infills & 2nd location
           ---3            get_infills(2, 3, false) returns infills & 3rd location
               ---4        get_infills(3, 4, false) returns infills & 4th location
        so merging the three results contains all 4 locations and no duplicates
 
       Example: for 2 locations where start and end are 200m apart:
                               • end
                           •   |
        diagonal_diff  •       |  northing_diff
                   •           |
         start •_______________|
                easting_diff

        • 3 infill coords are required
        • 5 coords are returned if include_start = true
    */

    // Build the output vec
    let mut coords: Vec<OSCoords> = Vec::new();

    if include_start {
        coords.push(coord_start);
    }

    // NB: work in floats for cumulative calcs to avoid rounding
    // innaccuracies which become noticeable over long distances

    // Get the diagonal difference between the start and end coords
    let easting_diff = coord_end.easting - coord_start.easting;
    let northing_diff = coord_end.northing - coord_start.northing;
    let diagonal_diff =
        ((easting_diff * easting_diff) as f64 + (northing_diff * northing_diff) as f64).sqrt();

    // Only create infills where the two locations are greater than 50m apart
    if diagonal_diff > ELEVATION_DISTANCE as f64 {

        // Get the infill easting & northing deltas
        // as a proportion of the infill diagonal diff
        let infill_diag_diff = diagonal_diff / ELEVATION_DISTANCE as f64;
        let delta_east = easting_diff as f64 / infill_diag_diff;
        let delta_north = northing_diff as f64 / infill_diag_diff;

        // Prepare an object to hold the generated infill location
        let mut infill_coords = OSCoords {
            easting: 0,
            northing: 0,
            elevation: None,
        };

        // Cumulativley add the delta_east & delta_north diffs
        // to create the required number of infill coords

        // Begin with the start location
        let mut cumulative_east = coord_start.easting as f64;
        let mut cumulative_north = coord_start.northing as f64;

        // Get the number of infills required   
        let infills_required = infill_diag_diff.ceil() as i64 - 1;

        // Create the infill locations
        for _ in 0..infills_required {

            cumulative_east += delta_east;
            cumulative_north += delta_north;

            // Store the infill location rounded to integer values
            infill_coords.easting = cumulative_east.round() as i64;
            infill_coords.northing = cumulative_north.round() as i64;
            coords.push(infill_coords);
        }
    }
    coords.push(coord_end);
    coords
}

pub fn parse_coords(input: &[&str]) -> Vec<OSCoords> {

    // Converts variously-styled input coordinates
    // to full grid origin coordinate pairs

    let mut clean_coords: Vec<OSCoords> = Vec::new();

    // Conversion multipliers for 500 Km² grid as [e, n]
    let grid_500: HashMap<&str, [i64; 2]> = [
        ("S", [0, 0]), ("T", [1, 0]),
        ("N", [0, 1]), ("O", [1, 1]),
        ("H", [0, 2]), ("J", [1, 2]),
    ].iter().cloned().collect();

    // Conversion multipliers for 100 Km² grid as [e, n]
    // NB: 'I' is not used
    let grid_100: HashMap<&str, [i64; 2]> = [
        ("V", [0, 0]), ("W", [1, 0]), ("X", [2, 0]), ("Y", [3, 0]), ("Z", [4, 0]),
        ("Q", [0, 1]), ("R", [1, 1]), ("S", [2, 1]), ("T", [3, 1]), ("U", [4, 1]),
        ("L", [0, 2]), ("M", [1, 2]), ("N", [2, 2]), ("O", [3, 2]), ("P", [4, 2]),
        ("F", [0, 3]), ("G", [1, 3]), ("H", [2, 3]), ("J", [3, 3]), ("K", [4, 3]),
        ("A", [0, 4]), ("B", [1, 4]),( "C", [2, 4]), ("D", [3, 4]), ("E", [4, 4]),
    ].iter().cloned().collect();

    for coord in input {

        let mut coords = OSCoords {
            easting: 0,
            northing: 0,
            elevation: None,
        };

        // Try to parse as alphanumeric coordinate
        let first_char = &coord[0..1];
        if grid_500.contains_key(first_char) {
            // Remove any spaces from the coordinate string because we can now treat it
            // as "AAn1n2" where n1 & n2 have an equal number of digits
            let coords_no_spaces = &coord.replace(" ", "");

            // Convert first char to full grid origin coordinates
            coords.easting += METRES_IN_500_GRID * grid_500[first_char][0];
            coords.northing += METRES_IN_500_GRID * grid_500[first_char][1];

            let second_char = &coord[1..2];
            if grid_100.contains_key(second_char) {
                // Convert 2nd char to full grid origin coordinates
                // and add to the 1st char coordinates
                coords.easting += METRES_IN_100_GRID * grid_100[second_char][0];
                coords.northing += METRES_IN_100_GRID * grid_100[second_char][1];

                if coords_no_spaces.len() > 2 {
                    // It's not just a 2 char string so try to split the
                    // remainder of the string in two parts
                    let remainder = &coords_no_spaces[2..];

                    let str_digit_pairs: Vec<String> = vec![
                        remainder[..remainder.len() / 2].to_string(),
                        remainder[remainder.len() / 2..].to_string(),
                    ];
                    // Get the numeric values and add to the full origin coordinates
                    let final_coords = get_full_coord_pair(str_digit_pairs);
                    coords.easting += final_coords.easting;
                    coords.northing += final_coords.northing;
                    clean_coords.push(coords);
                }
            } else {
                panic!("2nd letter in {} is invalid", &coord);
            }
        } else {
            // Not an alphanumeric coordinate string, so should just be numbers

            // First try to split on white space
            let coords_pair_space: Vec<&str> = coord.split_whitespace().collect();
            if coords_pair_space.len() == 2 {
                let mut vec_no_commas: Vec<String> = Vec::new();
                for str_coord in coords_pair_space {
                    // Remove any commas
                    vec_no_commas.push(str_coord.replace(",", ""));
                }
                coords = get_full_coord_pair(vec_no_commas);
                clean_coords.push(coords);
            } else {
                // No white space, so try to split on comma
                let coords_pair_comma: Vec<&str> = coord.split(',').collect();
                if coords_pair_comma.len() == 2 {
                    let mut vec_no_spaces: Vec<String> = Vec::new();
                    for str_coord in coords_pair_comma {
                        // Remove any commas
                        vec_no_spaces.push(str_coord.replace(" ", ""));
                    }
                    coords = get_full_coord_pair(vec_no_spaces);
                    clean_coords.push(coords);
                } else {
                    panic!("Cannot split numeric coordinate pairs");
                }
            }
        }
    }
    clean_coords
}

fn get_full_coord_pair(str_pair: Vec<String>) -> OSCoords {

    // Converts a vec of two numeric strings to five digit coordinates

    if str_pair.len() != 2 {
        panic!("Coordinate string must have 2 numeric values");
    }

    let mut coords = OSCoords {
        easting: 0,
        northing: 0,
        elevation: None,
    };

    for (i, item) in str_pair.iter().enumerate() {
        // Right-pad with zeros if required
        let str_digit = format!("{:0<5}", item);

        // Convert to i64
        if let Ok(number) = str_digit.parse::<i64>() {
            if i == 0 {
                coords.easting = number;
            } else if i == 1 {
                coords.northing = number;
            }
        } else {
            panic!("Coordinate {} is not a number", str_digit);
        }
    }
    coords
}

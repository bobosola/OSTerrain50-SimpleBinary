
use crate::utils;
use crate::os;
use std::{error::Error, path, fs, str};
use std::collections::HashMap;
use std::io::{Write, Seek, SeekFrom};
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{BufWriter};

// Code for writing the OS binary file

pub fn build_output_file(data_dir: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {

    const OUTPUT_FILE: &str = "OSTerrain50.bin";

    // Write the output file in directory which contains the data parent directory
    let output_file = utils::get_parent_dir(data_dir)?.join(path::Path::new(OUTPUT_FILE));
    let file = fs::File::create(&output_file)?;
    let mut file_buffer = BufWriter::new(file);

    // First write out the sig to identify the file type
    file_buffer.write_all(os::FILE_SIG)?;

    // Now write out the file header section.

    // This is a grid containing contiguous sets of 100 x 4 byte placeholders to hold the 
    // 4 byte addresses of the start of the elevation data for each of the 10km² data files
    // for each for the 91 100km² grids (SV, SW etc).

    // There can be anything from 0 to 100 data files for each section.
    // Where no data file exists, the address placeholder is left empty. 

    // Each section is preceded by an OS two letter identifier starting from the south-west
    // corner going E then N. This is done purely to help with debugging and sanity-checking the data
    // 1st: |SV|address1...address100| = 402 bytes 
    // 2nd: |SW|address1...address100| = 402 bytes
    // ...
    // 91st:|JM|address 1...address 100| = 402 bytes

    // A list of the full 7 x 13 100km² grid names
    let mut full_grid_list: Vec<String> = Vec::new();

    // Hash map to hold the addresses of the elevation data for each 10km² grid.
    let mut offsets = HashMap::new();  

    let mut grid_count_100k = 0;

    for i in 0..os::GRID_500.len() {
        let mut column = 0;
        for j in 0..os::GRID_100.len() {

            column += 1;
            // Processing grids S, N & H
            let grid_chars = vec![os::GRID_500[i][0], os::GRID_100[j]];
            file_buffer.write(&grid_chars)?;

            // Save the current grid identifier for later use when backfilling the header section 
            // with the actual data location addresses
            let grid_string = String::from_utf8(grid_chars).to_owned()?;
            full_grid_list.push(grid_string);         
            
            // Jump ahead 400 bytes to leave enough space for 100 four byte data addresses
            // which will be populated later once the header section has been created         
            file_buffer.seek(SeekFrom::Current(os::NUM_HEADER_ADDRESSES))?;            
            grid_count_100k += 1;

            // At the easternmost grid of each 500km² block add on the next two 100km² grids
            // from the adjacent 500km² block to complete the 7 data blocks from W to E.  
            if column == os::COLS_IN_500_GRID {
                for k in 0..=1 {

                    // e.g. where 'SZ' moves east to 'TV':
                    // 'S' has moved to a new 500km² grid to become 'T'
                    // so 'Z' must be re-wound back 4 columns to become 'V' 
                    // e.g. GRID_100[9] becomes GRID_100[9 + 0 - 4] ('SU' becomes 'TQ')
                    // then next loop it becomes GRID_100[9 + 1 - 4] ('TQ' becomes 'TR')
                    let new_position: usize = j + k - os::GRID_COLS_TO_REWIND;

                    // Processing grids T, O & J
                    let grid_chars = vec![os::GRID_500[i][1], os::GRID_100[new_position]];
                    file_buffer.write_all(&grid_chars)?;
                    let grid_string = String::from_utf8(grid_chars)?;
                    full_grid_list.push(grid_string);          

                    // Jump ahead again
                    file_buffer.seek(SeekFrom::Current(400))?;            
                    grid_count_100k += 1;                    
                }
                column = 0;

                if grid_count_100k == os::TOTAL_GB_GRIDS {
                    // Ignore grids north of row HL..JM
                    break;
                }            
            }
        }
    }

    // The header section is complete for now, so next step is read the data files.
    // The addresses of each data block will be written back to the header section
    // in the appropriate placeholder positions.

    // Get the path name for each of the unzipped OS data directories ('../data/hp' etc.)
    // by getting the first char from the 500 grid and the 2nd char from the 100 grid

    let grid_500: Vec<u8> = vec![
        os::GRID_500[0][0], os::GRID_500[0][1],
        os::GRID_500[1][0], os::GRID_500[1][1],
        os::GRID_500[2][0], os::GRID_500[2][1]                
    ];

    let mut file_count = 0;
    for first_char in grid_500.iter() {
        let first_char_vec = vec![*first_char];
        let char_1 = String::from_utf8(first_char_vec)?;

        for second_char in os::GRID_100.iter() {
            let second_char_vec = vec![*second_char];
            let char_2 = String::from_utf8(second_char_vec)?;

            let dir_name = format!("{}{}", char_1, char_2);

            // Create the full path to the data file directory (all directories are named in lower case)
            let match_dir = data_dir.join(path::Path::new(&os::INNER_DATA_DIR)).join(dir_name.to_lowercase());

            // No matching directory means no elevation data for this 10km² grid, so move on to the next one
            if !match_dir.is_dir() {
                continue;
            }

            // Work out the name for each of the possible (up to 100) 10km² data files
            // e.g. 'hp/HP00.asc',  'hp/HP01.asc' etc. inside the directory
            // NB: the data files are named in upper case but the directories are lower case.

            for file_num in 0..os::MAX_NUM_DATA_FILES {

                // All single digit file numbers are left-padded with a zero
                // to get a file name of e.g. 'HP03' where file_num is 3
                let file_name = format!("{}{:02}", dir_name, file_num);
                let file_name_with_suffix = format!("{}{}", file_name, os::FILE_SUFFIX);
                let file_path = match_dir.join(&file_name_with_suffix);

                if !file_path.is_file() {
                    // No data file exists (100% sea area)
                    continue;
                }
                file_count += 1;

                // Save the file name as e.g. 'HP00' along with the current offset address
                // for later use when back-filling the header section with data addresses
                let fp = file_buffer.seek(SeekFrom::Current(0))?;  
                offsets.insert(file_name.to_uppercase(), fp);

                // Read the file and split on each file line to get all the data rows.
                // The rows are suppled N to S and W to E
                //  │1 2 3       ... 200
                //  |201 202 203 ... 400 
                //	|...             40,000
                //  └──────────────	
                // These must be reversed to become S to N and W to E
                //	|...             40,000
                //  |201 202 203 ... 400
                //  │1 2 3       ... 200
                //  └──────────────	 
                // Then split each row to get the individual elevation values.
						
                println!("Processing file {:?}", file_path);

                let file_data = fs::read_to_string(file_path)?;
                let mut data_rows: Vec<&str> = file_data.split(os::OS_NEW_LINE).collect();
                data_rows.reverse();

                for data_row in data_rows {

                    // Get all the elevations in the row
                    let elevations: Vec<&str> = data_row.split(os::OS_DATA_SEPARATOR).collect();

                    // Ignore any metadata lines
                    if elevations.len() != os::ELEVATIONS_PER_ROW {
                        continue;
                    }

                    for elevation in elevations {

                        // Elevation values have either no decimal place or one (e.g. 23, 24.5)
                        // So multiply all by 10 to enable storage as u16 rather than f32.
                        let str_val_x10: String;
                        if elevation.contains('.') {
                            str_val_x10 = elevation.replace('.', "");
                        }
                        else {
                            str_val_x10 = format!("{}{}", elevation, "0");
                        }
         
                        // Write the x10 value to the file buffer as signed 16 bit integer
                        file_buffer.write_i16::<LittleEndian>(str_val_x10.parse::<i16>()?)?;
                    } 
                }    
            }
        }    
    } 
    println!("Processed {} data files", file_count);
    let data_points = file_count * os::ELEVATIONS_PER_ROW * os::ELEVATIONS_PER_ROW;
    println!("Processed {} elevation data points", utils::format_int(data_points as isize));
    
    // Now traverse the header placeholders in the same order as they were created
    // and write out the starting addresses of each data block for each area

    // Rewind the file pointer to just past the file sig
    file_buffer.seek(SeekFrom::Start(os::FILE_SIG.len() as u64))?;
 
    for grid_name in full_grid_list {

        // Jump over the the grid identifier
        file_buffer.seek(SeekFrom::Current(os::GRID_IDENT_LEN))?;

        // Fill the 400 byte header section with the data addresses going from W -> E 
        // then S -> N for each 4 byte data address for each of the found data files.
        // Skip over 4 bytes where there is no data file for that area.

        for northing in 0..os::ROWS_IN_10_GRID {
            for easting in 0..os::ROWS_IN_10_GRID {

                // Derive the data file identifier as stored in the hash map ('HP02' etc.)
                let identifier = format!("{}{}{}", grid_name, easting, northing);

                // If there is a data address for this area then write it out 
                // NB: the address is truncated from u64 to u32 to save space
                if offsets.contains_key(&identifier) {
                    file_buffer.write_u32::<LittleEndian>(offsets[&identifier] as u32)?;
                }
                else {
                    // No data for this area so jump ahead to leave a blank entry
                    file_buffer.seek(SeekFrom::Current(os::ADDRESS_LENGTH))?;
                }
            }
        }              
    }
    Ok(output_file) 
} 


use crate::utils;
use crate::os;
use std::{error::Error, path, fs, str};
use std::collections::HashMap;
use std::io::{Write, Seek, SeekFrom};
use byteorder::{LittleEndian, WriteBytesExt};

// Code for writing the OS binary file

pub fn build_output_file(data_dir: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {

    const OUTPUT_FILE: &str = "OSTerrain50.bin";

    // Write the file in directory which contains the data parent directory
    let output_file = utils::get_parent_dir(data_dir)?.join(path::Path::new(OUTPUT_FILE));
    let mut file = fs::File::create(&output_file)?;

    // First write out the sig to identify the file type
    file.write_all(os::FILE_SIG)?;

    // Now write out the file header section.

    // This is a grid containing contiguous sets of 100 x 4 byte placeholders to hold the 
    // 4 byte addresses of the start of the elevation data for each of the 10km² data files
    // for each for the 91 100km² grids (SV, SW etc).
    // There can be anything from 0 to 100 data files for each section.
    // Where no data file exists, the address placeholder is left empty. 

    // Each section is preceded by an OS two letter identifier starting from the south-west-most
    // corner going E then N. This is done purely to help with debugging and sanity-checking the data
    // 1st: |SV|address1...address100| = 402 bytes 
    // 2nd: |SW|addres1...address100| = 402 bytes
    // ...
    // 91st:|JM|address 1...address 100| = 402 bytes


    // Vector to hold a list of the full 7 x 13 100km² grid names
    let mut full_grid_list: Vec<String> = Vec::new();

    // Hash map to hold the addresses of the elevation data for each 10km² grid.
    let mut offsets = HashMap::new();  

    let mut grid_count_100k = 0;

    for i in 0..os::GRID_500.len(){
        let mut column = 0;
        for j in 0..os::GRID_100.len() {

            column += 1;
            let grid_chars = vec![os::GRID_500[i][0], os::GRID_100[j]];
            file.write_all(&grid_chars)?;  

            // Save the current grid identifier for later use when backfilling the header section 
            // with the actual data location addresses
            let grid_string = String::from_utf8(grid_chars)?.to_owned();
            full_grid_list.push(grid_string);         
            
            // Jump ahead 400 bytes to leave enough space for 100 x 4 byte data addresses
            // which will be populated later once the header section has been created         
            file.seek(SeekFrom::Current(400))?;            
            grid_count_100k += 1;

            // At the easternmost grid of each 500km² block add on the next two 100km² grids
            // from the adjacent 500km² block to complete the 7 data blocks from W to E.  
            if column == os::COLS_IN_500_GRID {
                for k in 0..=1 {
                    // e.g. where 'SZ' moves east to 'TV':
                    // 'S' has moved to a new 500km² grid to become 'T'
                    // so 'Z' must be re-wound back to start again at 'V' 
                    // e.g. GRID_100[4] becomes GRID_100[4 + k - 4] so 'Z' becomes 'V'
                    // and  GRID_100[9] becomes GRID_100[9 + k - 4] so 'U' becomes 'Q' etc.
                    let index: usize = j + k - os::GRID_COLS_TO_REWIND as usize;

                    let grid_chars = vec![os::GRID_500[i][1], os::GRID_100[index]];
                    file.write_all(&grid_chars)?;
                    let grid_string = String::from_utf8(grid_chars)?.to_owned();
                    full_grid_list.push(grid_string);          

                    // Jump ahead again
                    file.seek(SeekFrom::Current(400))?;            
                    grid_count_100k += 1;                    
                }
                column = 0;

                if grid_count_100k == os::GRIDS_WITH_DATA {
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

    for byte in grid_500.iter() {
        let v = vec ![*byte];
        let char_1 = String::from_utf8(v)?;

        for byte in os::GRID_100.iter() {
            let v = vec![*byte];
            let char_2 = String::from_utf8(v)?;

            let dir_name = format!("{}{}", char_1, char_2);

            // Create the full path to the data file directory (all directories are named in lower case)
            let match_dir = data_dir.join(path::Path::new(&os::INNER_DATA_DIR)).join(dir_name.to_lowercase());
            if !match_dir.is_dir(){
                // There is no elevation data for this 10km² grid, so move on to the next one
                continue;
            }

            // Work out the name for each of the possible (up to 100) 10km² data files
            // e.g. 'hp/HP00.asc',  'hp/HP01.asc' etc. inside the directory
            // NB: the data files are named in upper case

            for file_num in 0..os::MAX_NUM_DATA_FILES {

                // All single digit file numbers are left-padded with a zero
                // to get a file name of e.g. 'HP03' where file_num is 3
                let file_name = format!("{}{:02}", dir_name, file_num);
                let file_name_with_suffix = format!("{}{}", file_name, os::SUFFIX_ASC);
                let file_path = match_dir.join(&file_name_with_suffix);

                if !file_path.is_file(){
                    // No data file exists (100% sea area)
                    continue;
                }

                // Save the file name as e.g. 'HP00' along with the current offset address
                // This will later be used to store the address of the first data point for this file.
                let fp = file.seek(SeekFrom::Current(0))?;  
                offsets.insert(file_name.to_uppercase(), fp);

                // Read the file and split each line as a data row on the '\r\n'
                // Then reverse the datarows (i.e. last line read becomes first line written)            
                // Then split each data row on ' ' to get the individual elevation values.
                // so that the 40,000 data values are written from W > E and S > N thus:
                // 							
                //	|...             40,000
                //  |201 202 203 ... 400
                //  │1 2 3       ... 200
                //  └──────────────	 

                println!("Processing file {:?}", file_path);

                let file_data = fs::read_to_string(file_path)?;
                let mut data_rows: Vec<&str> = file_data.split("\r\n").collect();
                data_rows.reverse();

                for data_row in data_rows {

                    // The 200 elevations in the row are separated by a space
                    let elevations: Vec<&str> = data_row.split(" ").collect();

                    // Ignore any metadata lines
                    if elevations.len() != os::ELEVATIONS_PER_ROW as usize {
                        continue;
                    }

                    for elevation in elevations {

                        // Elevation values have either no decimal place or one (e.g. 23, 24.5)
                        // So multiply all by 10 to enable storage as u16 rather than f32.
                        let has_decimal_point = match elevation.find(".") {
                            Some(_) => true,
                            None => false
                        };

                        let str_val_x10: String;
                        if has_decimal_point {
                            str_val_x10 = elevation.replace(".", "");
                        }
                        else {
                            str_val_x10 = format!("{}0", elevation);
                        }
            
                        // Write the x10 value to the file as signed 16 bit integer
                        file.write_i16::<LittleEndian>(str_val_x10.parse::<i16>()?)?;
                    } 
                }             
            }
        }    
    } 
    
    // Now traverse the header placeholders in the same order as they were created
    // and write out the starting addresses of each data block for each area

    // Rewind the file pointer to just past the file sig
    file.seek(SeekFrom::Start(os::FILE_SIG.len() as u64))?;
 
    for grid_name in full_grid_list {

        // Jump over the two bytes containing the grid identifier
        file.seek(SeekFrom::Current(2))?;

        // Fill the 400 byte header section with the data addresses going from W -> E 
        // then S -> N for each 4 byte data address for the (up to) 100 data files
        // skipping over 4 bytes where there is no data file

        for northing in 0..os::ROWS_IN_10_GRID {
            for easting in 0..os::ROWS_IN_10_GRID {

                // Derive the data file identifier as stored in the hash map ('HP02' etc.)
                let identifier = format!("{}{}{}", grid_name, easting, northing);

                // If there is a data address for this area then write out the address 
                // truncated to 4 bytes to save space
                if offsets.contains_key(&identifier) {
                    file.write_u32::<LittleEndian>(offsets[&identifier] as u32)?;
                }
                else {
                    // No data for this area so jump ahead 4 bytes to leave a blank entry
                    file.seek(SeekFrom::Current(4))?;
                }
            }
        }              
    }
    Ok(output_file)  
}
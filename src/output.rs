
use crate::utils;
use crate::os;
use std::{path, fs, str};
use std::error::Error;
use std::collections::HashMap;
use std::io::{Write, Seek, SeekFrom, BufWriter};
use byteorder::{LittleEndian, WriteBytesExt};

// Code for writing the OS binary file

pub fn build_output_file(data_dir: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {

    // Create the output file in the same directory as the data parent directory
    let output_file = utils::get_parent_dir(data_dir)?.join(path::Path::new(os::OUTPUT_FILE_NAME));
    let file = fs::File::create(&output_file)?;

    // Buffer the file write
    let mut file_buffer = BufWriter::new(file);

    // Now write out the file header section.
    // This contains 91 contiguous sets of 402 bytes for grids SV to JM
    // which each contain:
    // - 1 two byte grid identifier
    // - 100 four byte address placeholders pointing to the locations of data.
    // This represents each 10km² grid for which there can be anything from
    // 0 to 100 data files. The address placeholder is left blank where
    // no data file exists. 

    // E.g. the data directory for SV has only 4 data files: SV80, SV81, SV90, SV91,
    // so the SV header section will contain 4 populated addresses and 96 blanks.
    // Each grid section is processed from E to W then N to S, so the SV section
    // will start with 8 blanks, then the SV80 address, then the SV90 address,
    // then 8 blanks, then the SV81 address, then the SV 91 address, with the rest
    // of the section all blanks.

    // The inclusion of the grid identifier is primarily for use with the 010 Editor
    // binary template to help navigate the ouput file for data checks and debugging.

    // Hash map to hold the addresses of the elevation data
    let mut offsets = HashMap::new();  

    // Create the header section
    
    // Write out the file signature
    file_buffer.write_all(os::FILE_SIG)?;

    // Write out the grid identifier then leave room for the 
    // mazimimum possible number of data addresses which will be populated later. 
    for grid in os::GRID_100.iter() {
        file_buffer.write_all(grid.as_bytes())?; 
        file_buffer.seek(SeekFrom::Current(os::MAX_NUM_DATA_FILES * os::ADDRESS_LENGTH))?;             
    }
    
    // End of header section (for now), so the data files can now be parsed and written out

    let mut file_count = 0;
    for grid in os::GRID_100.iter() {

        // Get the name for each of the data directories ('/hp' etc.)
        // NB: directories are named in lower case but files are named in upper case
        let dir_name = &grid.to_lowercase();

        // Create the full path to the data directory 
        let match_dir = data_dir.join(path::Path::new(&os::INNER_DATA_DIR)).join(dir_name);

        // No matching directory means no elevation data for this 10km² grid, so move on to the next one
        if !match_dir.is_dir() {
            continue;
        }

        // Calculate the numeric part of data file name for each of the possible data files
        // from E to E and N to S ('HP00.asc', HP01.asc' etc.).
        let mut file_identifers: Vec<String> = Vec::new();
        for first_num in 0..os::ROWS_IN_10_GRID {
            for second_num in 0..os::ROWS_IN_10_GRID {
                file_identifers.push(format!("{}{}", second_num, first_num))
            }
        }

        for file_num in file_identifers {

            let file_name = format!("{}{}", dir_name, file_num);

            // Check the file exists
            let file_path = match_dir.join(&format!("{}{}", file_name, os::FILE_SUFFIX));
            if !file_path.is_file() {
                continue;
            }
            file_count += 1;

            // Save the file name as e.g. 'HP00' along with the current offset address
            // for later use when back-filling the header section with data addresses
            let file_pointer = file_buffer.seek(SeekFrom::Current(0))?;  
            offsets.insert(file_name.to_uppercase(), file_pointer);

            // Read the file and split on each file line to get the data rows.
            // The data is suppled N to S and W to E
            //  │1 2 3       ... 200
            //  |201 202 203 ... 400 
            //	|...             40,000
            //  └──────────────	
            // These must be reversed to become S to N and W to E
            //	|...             40,000
            //  |201 202 203 ... 400
            //  │1 2 3       ... 200
            //  └──────────────	
                    
            println!("Processing file {:?}.", file_path);

            let file_data = fs::read_to_string(file_path)?;
            let mut data_rows: Vec<&str> = file_data.split(os::OS_NEW_LINE).collect();
            data_rows.reverse();

            for data_row in data_rows {

                let elevations: Vec<&str> = data_row.split(os::OS_DATA_SEPARATOR).collect();

                // Ignore any metadata lines
                if elevations.len() != os::ELEVATIONS_PER_ROW {
                    continue;
                }

                for elevation in elevations {

                    // Elevation values have either no decimal place or one, so multiply
                    // all values by 10 to enable storage as i16 rather than f32.
                    let str_val_x10: String;
                    if elevation.contains('.') {
                        str_val_x10 = elevation.replace('.', "");
                    }
                    else {
                        str_val_x10 = format!("{}{}", elevation, "0");
                    }
        
                    // Write the x10 value as signed 16 bit integer
                    file_buffer.write_i16::<LittleEndian>(str_val_x10.parse::<i16>()?)?;
                } 
            }    
        }
    }    
  
    // Now traverse the header section again and write out the starting addresses of
    // each data block for each grid section

    // Rewind the file pointer to just past the file sig
    file_buffer.seek(SeekFrom::Start(os::FILE_SIG.len() as u64))?;
 
    for grid in os::GRID_100.iter() {

        // Jump over the the grid identifier
        file_buffer.seek(SeekFrom::Current(os::GRID_IDENT_LEN))?;

        // Fill the 400 byte header section with the data addresses going from W -> E 
        // then S -> N for each 4 byte data address for each of the found data files.
        // Skip over 4 bytes where there is no data file for that area.

        for northing in 0..os::ROWS_IN_10_GRID {
            for easting in 0..os::ROWS_IN_10_GRID {

                // Derive the data file identifier ('HP01' etc.)
                let identifier = format!("{}{}{}", grid, easting, northing);

                // If there is a stored data address for this area then write it out 
                // NB: the address is truncated from u64 to u32 to save space.
                if offsets.contains_key(&identifier) {
                    file_buffer.write_u32::<LittleEndian>(offsets[&identifier] as u32)?;
                }
                else {
                    // No data for this area so leave a blank entry
                    file_buffer.seek(SeekFrom::Current(os::ADDRESS_LENGTH))?;
                }
            }
        }              
    }

    file_buffer.flush()?;

    println!("Processed {} OS data files.", utils::format_int(file_count));
    let data_points = file_count as usize * os::ELEVATIONS_PER_ROW * os::ELEVATIONS_PER_ROW;
    println!("Processed {} elevation data points.", utils::format_int(data_points as isize));

    Ok(output_file) 
} 

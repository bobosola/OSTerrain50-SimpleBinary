
use crate::utils;
use crate::os;
use std::{path, fs, str};
use std::error::Error;
use std::collections::HashMap;
use std::io::{Write, Seek, SeekFrom, BufWriter};

// Code for writing the OS binary file

pub fn build_output_file(data_dir: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {

    // Create the output file in the same directory as the data parent directory
    // and open a file buffer for writing the content.
    // See the README.md for a full description of the file format
    let output_file = utils::get_parent_dir(data_dir)?.join(path::Path::new(os::OUTPUT_FILE_NAME));
    let file = fs::File::create(&output_file)?;
    let mut file_buffer = BufWriter::new(file);

    /***********************************************************************
       Write out the header section
    ************************************************************************/

    // Hash map to hold the grid identifier and data addresses of the elevation data blocks
    let mut offsets = HashMap::new();  

    // Write the file signature
    file_buffer.write_all(os::FILE_SIG)?;

    // Write the grid identifiers followed by enough space for the maximimum possible
    // number of data addresses (to be populated later). 
    for grid in os::GRID_100.iter() {
        file_buffer.write_all(grid.as_bytes())?; 
        file_buffer.seek(SeekFrom::Current(os::MAX_NUM_DATA_FILES * os::ADDRESS_LENGTH))?;             
    }
    
    /***********************************************************************
       Read the data files and write the elevations to the output
    ************************************************************************/    

    let mut file_count = 0;
    for grid in os::GRID_100.iter() {

        // Get the name for each of the OS data directories (hp etc.)
        // NB: directories are named in lower case but files are named in upper case
        let dir_name = &grid.to_lowercase();

        // Create the full path to the data directory 
        let match_dir = data_dir.join(path::Path::new(&os::INNER_DATA_DIR)).join(dir_name);

        // Check the directory exists. No directory means no elevation data for this 100km² grid.
        if !match_dir.is_dir() {
            continue;
        }

        // Calculate the numeric part of data file name for each of the possible data files
        // ordered from E to W and N to S (e.g. xx00, xx10, xx20 ... xx79, xx89, xx99)
        let mut file_identifers: Vec<String> = Vec::new();
        for northing in 0..os::ROWS_IN_10_GRID {
            for easting in 0..os::ROWS_IN_10_GRID {
                file_identifers.push(format!("{}{}", easting, northing))
            }
        }
        for file_num in file_identifers {

            // Combine with the directory name (to get e.g. hp01)
            let file_name = format!("{}{}", dir_name, file_num);

            // Add the file suffix and check the file exists
            let file_path = match_dir.join(&format!("{}{}", file_name, os::FILE_SUFFIX));

            if !file_path.is_file() {
                continue;
            }
            file_count += 1;

            // Save the file name (e.g. HP00) along with the current offset address
            // for later use when back-filling the header section
            let file_pointer = file_buffer.seek(SeekFrom::Current(0))?;  
            offsets.insert(file_name.to_uppercase(), file_pointer);

            // Each data file is a CSV-like format using spaces instead of commas.
            // They start with some metadata lines then have 200 data rows each 
            // containing 200 elevations.

            // The data rows in each file are suppled N to S and W to E
            // These must be reversed to be stored as S to N while remaining W to E
                  
            // NB: various crates were tried here (e.g. CSV and more) but none easily
            // supported reversing the data rows and/or had problems handling the metadata
            // lines which are shorter than the data lines. Hence the code below.

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
        
                    // Write the x10 value as a signed 16 bit little endian integer
                    let i16_bytes = str_val_x10.parse::<i16>()?.to_le_bytes();
                    file_buffer.write_all(&i16_bytes)?;
                } 
            }    
        }
    }   
    
    /************************************************************************
       Re-traverse the header section and write out the saved data addresses
    *************************************************************************/    
  
    // Rewind the file pointer to just past the file sig
    file_buffer.seek(SeekFrom::Start(os::FILE_SIG.len() as u64))?;
 
    for grid in os::GRID_100.iter() {

        // Jump over the the grid identifier
        file_buffer.seek(SeekFrom::Current(os::GRID_IDENT_LEN))?;

        // Fill up the header block for this grid with the stored data addresses
        // leaving a blank address where there is no matching data address
        for northing in 0..os::ROWS_IN_10_GRID {
            for easting in 0..os::ROWS_IN_10_GRID {

                // Derive the data file identifier (HP01 etc.)
                let identifier = format!("{}{}{}", grid, easting, northing);

                // Look up the data address in the hash map then write it out
                // converted to little endian u32 to fit the four-byte placeholder
                if offsets.contains_key(&identifier) {
                    let u32_bytes = (offsets[&identifier] as u32).to_le_bytes();
                    file_buffer.write_all(&u32_bytes)?;
                }
                else {
                    // No data for this area so skip to the next placeholder
                    file_buffer.seek(SeekFrom::Current(os::ADDRESS_LENGTH))?;
                }
            }
        }              
    }
    file_buffer.flush()?;
    println!("Processed {} OS data files.", utils::format_int(file_count));
    Ok(output_file) 
} 
mod output;
mod unzip;
mod utils;
mod os;

use std::{env, error::Error, path, string};

// Converts Ordnance Survey 'OS Terrdata 50' ASCII data to a binary format
// Args are either:
// - an OS data zip file containing the zipped elevation data, or
// - a directory containing already-unzipped elevation data

struct ArgsTypes {
    zip_file: Option<path::PathBuf>,
    directory: Option<path::PathBuf>,
}

fn main() {

    let args: Vec<_> = env::args().collect();

    // Sanity-check the supplied args
    match args_check(&args) {
        Ok(args) => {
            let mut data_dir = path::PathBuf::new();

            // If it's a zip file then unzip it in its parent directory            
            match args.zip_file {
                Some(filepath) => {
                    match utils::get_parent_dir(&filepath){
                        Ok(parent) => {  

                            // Unzipping returns the topmost directory of the unzipped archive                    
                            match unzip::unzip_os_file(&filepath, &parent) {
                                Ok(unzipped_top_dir) => data_dir = unzipped_top_dir,
                                Err(e) => utils::die(&e)
                            }
                        }
                        Err(e) => utils::die(&e)
                    }
                }
                _ => ()
            }
            // Got a directory as args (presumed to contain unzipped data)            
            match args.directory {
                Some(dir) => data_dir = dir,
                _ => (),
            }
            // In both cases then build the output file from the data directory
            match output::build_output_file(&data_dir) {
                Ok(output_path) => println!("Binary data file {:?} created", output_path),
                Err(e) => utils::die(&e),
            }
        }
        Err(e) => utils::die(&e)
    }
}

fn args_check(args: &Vec<string::String>) -> Result<ArgsTypes, Box<dyn Error>> {
    
    let mut args_types = ArgsTypes {
        zip_file: None,
        directory: None,
    };

    if args.len() == 2 {
        let arg = path::Path::new(&args[1]);
        if unzip::is_zip_file(arg) {
            args_types.zip_file = Some(arg.to_path_buf());
        }
        else if arg.is_dir() {
            args_types.directory = Some(arg.to_path_buf());
        }
        else {
            Err("The argument was not a valid zip file or directory")?
        }
    }

    if args_types.directory == None && args_types.zip_file == None {
        show_args_usage(&args[0])?;
        Err("Invalid or missing argument")?
    }
    Ok(args_types)
}

fn show_args_usage(arg: &str) -> Result<(), Box<dyn Error>>{
    
    // Get the app name from the args[0] app path
    let app_path = path::Path::new(&arg);
    let app_name = app_path.file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Could not convert app fikle name to string")?;
    eprint!(
"
Usage:
1) {} <OS zip file> : Unzips OS data file to a data directory and creates binary output file
2) {} <directory>   : Uses an existing data directory to create binary output file

" , app_name, app_name);    
    Ok(())
}
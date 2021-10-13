mod output;
mod unzip;
mod utils;
mod os;

use std::{env, path, string};
use std:: error::Error;
use std::time::Instant;

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
    let start_time = Instant::now();

    // Sanity-check the supplied args
    match args_check(&args) {
        Ok(args) => {

            // If it's a zip file, unzip it in its parent directory
            // and return the path to the unzipped files directory      
            let mut data_dir = path::PathBuf::new(); 
            
            if let Some(zipfile) = args.zip_file {
                match utils::get_parent_dir(&zipfile){
                    Ok(parent) => {                  
                        match unzip::unzip_os_file(&zipfile, &parent) {
                            Ok(unzipped_top_dir) => data_dir = unzipped_top_dir,
                            Err(e) => utils::die(e)
                        }
                    }
                    Err(e) => utils::die(e)
                }                
            }

            // Got a directory as args (presumed to contain unzipped data)  
            if let Some(dir) =  args.directory {
                data_dir = dir;
            }         

            // Build the output file from the data directory
            match output::build_output_file(&data_dir) {
                Ok(output_path) => println!("Binary data file {:?} created.", output_path),
                Err(e) => utils::die(e),
            }
        }
        Err(e) => utils::die(e)
    }
    println!("Completed in {:.2?} seconds.", start_time.elapsed());
}

fn args_check(args: &[string::String]) -> Result<ArgsTypes, Box<dyn Error>> {
    
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
            return Err("The argument was not a valid zip file or directory".into())
        }
    }

    if args_types.directory == None && args_types.zip_file == None {
        show_args_usage(&args[0])?;
        return Err("Invalid or missing argument".into())
    }
    Ok(args_types)
}

fn show_args_usage(arg: &str) -> Result<(), Box<dyn Error>>{
    
    // Get the app name from the args[0] app path
    let app_path = path::Path::new(&arg);
    let app_name = app_path.file_name()
        .and_then(|s| s.to_str())
        .ok_or( "Could not convert app fikle name to string")?;
    eprint!(
"
Usage:
1) {} <OS zip file> : Unzips an OS Terrain 50 data zip file then creates 
                               a binary data file from the unzipped data directory.

2) {} <directory>   : Creates a binary data file from an OS Terrain 50 data directory.

" , app_name, app_name);    
    Ok(())
}
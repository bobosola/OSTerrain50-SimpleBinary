mod zip;
mod binaryfile;

use std::{process, path, env, string};
use std::error::Error;

struct ArgsType {
    zip_file: Option<path::PathBuf>,
    extracted_dir: Option<path::PathBuf>,
}

fn main() {

    let args: Vec<_> = env::args().collect();
    match args_check(&args){
        Ok(args) => {
            match args.zip_file {
            // Argument is a zip file, so unzip it and all child zips                
                Some(f) => {             
                    match zip::unzip_os_file(&f) { 
                        // The unzipping returns the data files directory path
                        // so we can now build the output binary
                        Ok(data_dir) => binaryfile::build_output_file(&data_dir),
                        Err(e) => {
                            eprintln!("Error: {:?},", e);
                            process::exit(1);
                        }
                    }
                }
                None => ()
            }
            match args.extracted_dir{
            // Argument is a directory assumed to contain unzipped data files
            // so try to build the output binary                        
                Some(data_dir) => {
                    binaryfile::build_output_file(&data_dir);
                }
                None => ()
            }
        }         
        Err(err) => {
            eprintln!("Error: {:?}", err);
            process::exit(1);
        }
    } 
}

fn args_check(args: &Vec<string::String>) -> Result<ArgsType, Box<dyn Error>> {

    if args.len() < 2 {
        eprint!("
Usage:
1) {} <OS zip file> : Unzips the OS data file and creates the binary output file
2) {} <directory>   : Creates the binary output file where <directory> contains
the unzipped OS data files

", args[0], args[0]);
       Err("Incorrect arguments")?
    }

    let mut args_type = ArgsType {
        zip_file: None,
        extracted_dir: None,
    };    

    if args.len() == 2 {
        let args_path = path::Path::new(&args[1]); 
        if args_path.is_file() {
            match args_path.to_str() {
                Some(file) => {
                    if file.ends_with(".zip") {
                        args_type.zip_file = Some(args_path.to_path_buf());
                    }
                    else {
                        Err("The file is not a zip file")?
                    }
                }
                None => Err("The file name is not a valid string")?
            };        
        }
        if args_path.is_dir() {
            args_type.extracted_dir = Some(args_path.to_path_buf());
        }      
    }
    Ok(args_type)
}

mod output;
mod unzip;

use std::{env, error::Error, path, process, string};

// Converts Ordnance Survey (OS) data to a custom binary format
// Args are either:
// - an OS data zip file containing the zipped elevation data, or
// - a directory containing unzipped elevation data files

struct ArgsTypes {
    zip_file: Option<path::PathBuf>,
    directory: Option<path::PathBuf>,
}

fn main() {

    let args: Vec<_> = env::args().collect();
    match args_check(&args) {
        Ok(args) => {
            let mut data_dir = path::PathBuf::new();
            match args.zip_file {
                Some(f) => {
                    match unzip::unzip_os_file(&f) {
                        // The unzipping operation returns the data files directory path
                        Ok(d) => data_dir = d,
                        Err(e) => die(&e),
                    }
                }
                None => (),
            }
            match args.directory {
                Some(d) => data_dir = d,
                None => (),
            }

            // Build the output file using the data directory
            match output::build_output_file(&data_dir) {
                Ok(f) => println!("Data file {:?} created", f),
                Err(e) => die(&e),
            }
        }
        Err(e) => die(&e),
    }
}

fn args_check(args: &Vec<string::String>) -> Result<ArgsTypes, Box<dyn Error>> {
    
    let mut args_types = ArgsTypes {
        zip_file: None,
        directory: None,
    };

    if args.len() == 2 {
        let args_path = path::Path::new(&args[1]);
        if args_path.is_file() {
            match args_path.to_str() {
                Some(f) => {
                    if f.ends_with(".zip") {
                        args_types.zip_file = Some(args_path.to_path_buf());
                    } else {
                        Err("The file is not a zip file")?
                    }
                }
                None => Err("The argument is not a valid path")?,
            };
        }
        if args_path.is_dir() {
            args_types.directory = Some(args_path.to_path_buf());
        }
    }

    if args_types.directory == None && args_types.zip_file == None {
        eprint!(
            "
Usage:
1) {} <OS zip file> : Unzips OS data file to a data directory and creates binary output file
2) {} <directory>   : Uses an existing data directory to create binary output file

",
            args[0], args[0]
        );
        Err("No argument supplied")?
    }
    Ok(args_types)
}

fn die(err: &Box<dyn Error>) {
    eprintln!("Error: {},", &err);
    process::exit(1);
}
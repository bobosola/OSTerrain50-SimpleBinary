mod zip;
mod binaryfile;

use std::{process, path, env};

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage 1: {} <OSTerrain50 zip file>", args[0]);
        eprintln!("Usage 2: {} -nounzip <'data' dir containing subdirectories of unzipped files>", args[0]);     
        process::exit(1);
    }

    if args.len() == 3 && &args[1] == "-nounzip"{
        binaryfile::build_binary_file(&args[2]);
    }
    else if args.len() == 2 {

        // The OS zip will be extracted to a directory named the same
        // as the zip file (minus the extension) in the application directory
        let os_zip_file_path = path::Path::new(&args[1]);    
        let extract_dir_name  = os_zip_file_path.file_stem().unwrap(); 

        eprintln!("Extracting {}", os_zip_file_path.to_str().unwrap());        
        eprintln!("Extraction directory is {}", extract_dir_name.to_str().unwrap());    

        let current_dir = env::current_dir().unwrap(); 
        let result = zip::unzip(os_zip_file_path, &current_dir, true);
        match result {
            Ok(count) => eprintln!("Extraction contains {} child zips", count),
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        };

        // Inside the main directory is a subdirectory called "data" which contains
        // all the subdirectories named as per OS grids (SV, SW etc.). These contain
        // a variable number of child data zips for that grid

        const DATA_DIR: &str=  "data";
        let data_path = current_dir.join(extract_dir_name).join(DATA_DIR);    

        // now we can unzip all the child zips
        let result = zip::unzip_subdirs(&data_path);
        match result {
            Ok(count) => eprintln!("Extracted {} zip files", count),
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        };
    }
}

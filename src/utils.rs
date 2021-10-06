use std::{error::Error, path, process, string};

// Utility functions

pub fn die(err: Box<dyn Error>) {

    eprintln!("Error: {}", &err);
    process::exit(1);
}

pub fn get_parent_dir(path: &path::Path) ->  Result<path::PathBuf, Box<dyn Error>>{

    if let Some(path) = path.parent(){
        if path.is_dir() {
            return Ok(path.to_path_buf());
        }
    }
    Err("Could not get parent directory".into())
}

// Formats an integer with commas for readability
pub fn format_int(int_to_fix: isize) -> string::String {

    let mut output = String::new();
    let int_string = int_to_fix.to_string();

    let all = int_string.chars().rev().enumerate();
    for (i, val) in all {
        if i != 0 && i % 3 == 0 {
            output.insert(0, ',');
        }
        output.insert(0, val);
    }
    output
}
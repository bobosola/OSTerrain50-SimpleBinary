use std::{error::Error, path, process, string};

/***********************************************************************
   Utility functions
************************************************************************/

pub fn die(err: Box<dyn Error>) {
    eprintln!("Error: {}", &err);
    process::exit(1);
}

pub fn get_parent_dir(path: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {
    if let Some(p) = path.parent() {
        if p.is_dir() {
            return Ok(p.to_path_buf());
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

#[test]
fn format_number_with_commas() {
    assert_eq!("1,000".to_string(), format_int(1_000));
    assert_eq!("1,000,000".to_string(), format_int(1_000_000));
    assert_eq!("1,000,000,000".to_string(), format_int(1_000_000_000));
}

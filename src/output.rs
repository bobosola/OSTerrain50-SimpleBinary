use std::{error::Error, path};

/*
    All the OS binary file writing code
*/

pub fn build_output_file(data_dir: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {
    eprintln!("data dir is {:?}", data_dir);
    Ok(data_dir.to_path_buf())
}

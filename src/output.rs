use std::{error::Error, path};

// All code related to writing the OS binary file

pub fn build_output_file(data_dir: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {
    eprintln!("Data directory: {:?}", data_dir);
    Ok(data_dir.to_path_buf())
}

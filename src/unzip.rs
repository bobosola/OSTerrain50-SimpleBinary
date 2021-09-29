use std::{error::Error, fs, io, path};
use walkdir::WalkDir;

 // All the code related to unzipping the downloaded OS data file

pub fn unzip_os_file(file_path: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {
    
    // The OS zip file contains embedded instructions to be extracted to a directory
    // named the same as the zip file name minus the '.zip' suffix.
    // Inside this directory is a subdirectory called "data" which contains 
    // subdirectories named as per all the OS grids (HP, HT etc.).
    // These each contain a variable number of data zips which contain the
    // actual elevation data ASCII files, like this:
    //
    //   target_dir/zip_file_name_dir/data/HP/HP01.asc
    //   target_dir/zip_file_name_dir/data/HP/HP02.asc    
    //   etc.

    // Extract the zip file into the same directory as the zip file
    let target_dir = file_path.parent()
        .ok_or_else(|| "Could not obtain the zip file parent directory")?;

    println!(
        "Extracting {}", file_path.to_str()
        .ok_or_else(|| "Zip file path is an invalid string")?
    ); 

    let count = unzip(file_path, &target_dir)?;
    println!("Found {} zip files", count);

    // Unzip all the child zips inside their parent directories
    let top_unzip_dir = file_path.file_stem()
        .ok_or_else(|| "Could not obtain the zip file stem")?;  
    let data_dir = target_dir.join(top_unzip_dir).join("data");    
    let count = unzip_subdirs(&data_dir)?;
    
    println!("Extracted {} zip files", count);
    Ok(data_dir)
}

fn unzip( source: &path::Path, target_dir: &path::Path ) -> Result<u64, Box<dyn Error>> {

    // Modified from demo code at https://github.com/zip-rs/zip/blob/master/examples/extract.rs

    let file = fs::File::open(&source)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let file_count = archive.len() as u64;

    for i in 0..archive.len() {

        // The archive entry may contain a directory heirarchy   
        // to be extracted to, so check any such path is valid, otherwise ignore
        let mut entry = archive.by_index(i)?;
        let outpath = match entry.enclosed_name() {
            Some(path) => target_dir.join(path.to_owned()),
            None => continue,
        };

        // Create any specified directory entries
        if (&*entry.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            // Create a file entry's parent directories
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = fs::File::create(&outpath)?;

            // If the zip file has no specified parent directory
            // then unzip it inside the parameter target dir        
            if let Some(p) = outpath.parent() {
                if p != target_dir {
                    outfile = fs::File::create(&target_dir.join(entry.name()))?;
                }
            }
            io::copy(&mut entry, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            // Get and set permissions where applicable
            if outpath.exists() {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = entry.unix_mode() {
                    fs::set_permissions(outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }
    }
    Ok(file_count)
}

fn unzip_subdirs(data_dir: &path::Path) -> Result<u64, Box<dyn Error>> {

    // Rescursively examine all subdirectories and unzip each zip file found
    // into its containing directory, then delete the zip file

    let walker = WalkDir::new(data_dir).into_iter();
    let mut unzip_count = 0;

    for dir_entry in walker {

        let de = dir_entry?;
        let found_object = de.path();

        if is_zip_file(found_object) { 
            println!("Extracting {}", found_object.display());

            let zip_dir = found_object.parent()
                .ok_or_else(|| "Could not determine parent")?;

            let result = unzip(found_object, &zip_dir);
            match result {
                Ok(_) => {
                    unzip_count += 1;
                    fs::remove_file(found_object)?;
                }
                Err(err) => Err(err)?
            };
        }
    }
    Ok(unzip_count)
}

pub fn is_zip_file(path: &path::Path) -> bool {

    let has_zip_suffix = match path.to_str() {
        Some(path) => {
            if path.ends_with(".zip") {
                true
            }
            else {
                false
            }
        }
        None => false
    };

    if has_zip_suffix && path.is_file() {
        return true;
    }
    false
}
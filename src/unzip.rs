use std::{error::Error, fs, io, path};
use walkdir::WalkDir;

 // Code for handling zip files

pub fn unzip_os_file(file_path: &path::Path, target_dir: &path::Path,) -> Result<path::PathBuf, Box<dyn Error>> {
    
    println!(
        "Extracting {}", file_path.to_str()
        .ok_or_else(|| "Zip file path is an invalid string")?
    ); 
    let unzipped_top_dir = unzip(file_path, &target_dir)?;

    // Unzip all the nested child zips inside their parent directories   
    let count = unzip_subdirs(&unzipped_top_dir)?;
  
    println!("Extracted {} zip files", count);
    Ok(unzipped_top_dir)
}

fn unzip( source: &path::Path, target_dir: &path::Path ) -> Result<path::PathBuf, Box<dyn Error>> {

    // Modified from demo code at https://github.com/zip-rs/zip/blob/master/examples/extract.rs

    let file = fs::File::open(&source)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut top_dir = path::PathBuf::new();

    for i in 0..archive.len() {

        // The archive entry may contain a directory heirarchy   
        // to be extracted to, so check any such path is valid, otherwise ignore
        let mut entry = archive.by_index(i)?;
        let outpath = match entry.enclosed_name() {
            Some(p) => target_dir.join(p.to_owned()),
            None => continue,
        };

        if i == 0 {
            // Get the top level directory of the unzipped archive
            top_dir = outpath.clone();
        }         

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

            // If the zip file has no specified parent path
            // then unzip it inside the parameter target directory       
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
    Ok(top_dir)
}

fn unzip_subdirs(data_dir: &path::Path) -> Result<u64, Box<dyn Error>> {

    // Recursively examine all subdirectories and unzip each zip file found
    // into its containing directory, then delete the zip file.
   
    let walker = WalkDir::new(data_dir).sort_by_file_name().into_iter();
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
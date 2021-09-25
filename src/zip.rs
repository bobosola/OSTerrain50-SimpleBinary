use std::{process, io, fs, path};
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::{WalkDir};

/*
    All the zip-related code
*/

pub fn unzip(
    source_path: &path::Path,
    dest_path: &path::Path,    
    show_progress_bar: bool
    ) -> Result<u64, Box<dyn Error>> {

    let file = fs::File::open(&source_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let file_count = archive.len() as u64;

    let mut bar = ProgressBar::new(0);
    if show_progress_bar {
        bar = ProgressBar::new(file_count);
        bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
        .progress_chars("##-"));
    }
 
    for i in 0..archive.len() {

        if show_progress_bar {
            bar.inc(1);
        }
        
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile;
            if outpath.parent().unwrap() != dest_path {
                outfile = fs::File::create(&dest_path.join(file.name()))?;
            }
            else {
                outfile = fs::File::create(&outpath)?;
            }

            io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            // Get and Set permissions where applicable
            if outpath.exists() {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    } 
    Ok(file_count)
}

pub fn unzip_subdirs(data_dir: &path::Path) -> Result<u64, Box<dyn Error>> {

    // Recursively examin all subdirectories and unzip each file
    // deleting the zip on completion

    eprintln!("data dir: {}", data_dir.display());
    let walker = WalkDir::new(data_dir).into_iter();
    let mut zip_count = 0;
    for entry in walker {
        let f = entry?;
        let path = f.path();
        if path.is_file() && path.to_str().unwrap().ends_with(".zip") {
            zip_count += 1;
            eprintln!("Extracting {}", path.display());
            let result = unzip(path, path.parent().unwrap(), false);
            match result {
                // Successful extraction so delete the zip file
                Ok(_) => fs::remove_file(path)?,
                Err(error) => {
                    eprintln!("Error: {}", error);
                    process::exit(1);
                }
            };
        }
    }    
    Ok(zip_count)
}
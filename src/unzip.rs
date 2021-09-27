use indicatif::{ProgressBar, ProgressStyle};
use std::{env, error::Error, fs, io, path};
use walkdir::WalkDir;

/*
    All the unzip-related code
*/

pub fn unzip_os_file(file_path: &path::Path) -> Result<path::PathBuf, Box<dyn Error>> {
    
    // The OS zip will be extracted to a directory named the same
    // as the zip file (minus the extension) in the application directory
    let extract_dir_name = file_path.file_stem().unwrap();

    eprintln!("Extracting {:?}", Some(file_path.to_str()));
    eprintln!("Extraction directory is {:?}", extract_dir_name);

    let current_dir = env::current_dir()?;
    let count = unzip(file_path, &current_dir, true)?;
    eprintln!("Found {} zip files", count);

    // Inside the extraction directory is a subdirectory called "data" which contains
    // subdirectories named as per all the OS grids (SV, SW, etc.). These each contain
    // a variable number of child data zips

    const DATA_DIR: &str = "data";
    let data_path = current_dir.join(extract_dir_name).join(DATA_DIR);

    // Unzip all the child zips inside their parent directories
    let count = unzip_subdirs(&data_path)?;
    eprintln!("Extracted {} zip files", count);

    Ok(data_path)
}

fn unzip(
    source: &path::Path,
    dest: &path::Path,
    show_progress_bar: bool,
) -> Result<u64, Box<dyn Error>> {
    let file = fs::File::open(&source)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let file_count = archive.len() as u64;

    let mut bar = ProgressBar::new(0);
    if show_progress_bar {
        bar = ProgressBar::new(file_count);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
                .progress_chars("##-"),
        );
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
            if outpath.parent().unwrap() != dest {
                outfile = fs::File::create(&dest.join(file.name()))?;
            } else {
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
    // Examine all subdirectories and unzip each file
    // then delete the zip on completion

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
                Ok(_) => fs::remove_file(path)?,
                Err(err) => Err(err)?,
            };
        }
    }
    Ok(zip_count)
}
mod util;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::Result;

use crate::errors::FontError;

const INSTALL_PATH: &str = ".fonts";

#[derive(Debug, Copy, Clone)]
pub(crate) struct ExtractOptions {
    pub(crate) delete_zip: bool,
    pub(crate) use_otf: bool,
    pub(crate) interactive: bool,
}

pub(crate) fn extract_fonts_from_zip(
    zip_path: &Path,
    font_name: &str,
    opts: ExtractOptions,
) -> Result<u32> {
    log::debug!("Starting to unzip");
    let file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    log::debug!("unziped");
    let mut installed = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let new_path = append_font_dir(&path, font_name)?;
                new_path.to_owned()
            }
            None => continue,
        };

        let file_name = file.name();

        if file_name.ends_with('/') {
            log::info!("Extracting directory in: \"{}\"", outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            if util::should_ignore(opts, file_name, &outpath) {
                log::info!("{} ignored", file_name);
                continue;
            }
            log::info!(
                "Extracting file in: \"{}\" ({} bytes)",
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
        installed += 1;
    }
    if opts.delete_zip {
        fs::remove_file(zip_path)?;
    }
    Ok(installed)
}

pub(crate) fn remove_font_dir(font_name: &str) -> Result<()> {
    let home = home::home_dir().ok_or(FontError::HomeNotFound)?;
    let path = home.join(INSTALL_PATH).join(font_name);
    fs::remove_dir_all(path)?;
    Ok(())
}

fn append_font_dir(p: &Path, d: &str) -> Result<PathBuf> {
    let home = home::home_dir().ok_or(FontError::HomeNotFound)?;
    // Safe to unwrap as the file_name is ensured to exist before.
    let file_name = p.file_name().unwrap();
    match p.parent() {
        Some(dirs) => Ok(home.join(INSTALL_PATH).join(d).join(dirs).join(file_name)),
        None => Ok(home.join(INSTALL_PATH).join(d).join(file_name)),
    }
}

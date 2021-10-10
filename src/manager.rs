use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use log::{error, info};

use crate::errors::FontError;
use crate::files::extract_fonts_from_zip;
use crate::files::remove_font_dir;
use crate::files::ExtractOptions;

pub(crate) async fn download_zip<'a, 'b>(url: &'a str, fname: &'b str) -> Result<&'b Path> {
    info!("Downloading: {}", url);
    let response = reqwest::get(url).await?;
    let response = response.error_for_status()?;
    let path = Path::new(fname);
    let mut file = File::create(path)?;
    file.write_all(&response.bytes().await?)?;
    Ok(path)
}

pub(crate) async fn install_from_url(url: &str, opts: ExtractOptions) -> Result<()> {
    // safe to unwrap, A valid url must have at least one /
    let fname = url.split('/').last().unwrap();
    let fname = format!("./{}", fname);
    let path = download_zip(url, &fname).await?;
    let fname = path
        .file_stem()
        .ok_or(FontError::InvalidPath)?
        .to_str()
        .ok_or(FontError::InvalidPath)?;
    let installed = extract_fonts_from_zip(path, fname, opts)?;
    info!("{} ttf or otf fonts installed!", installed);
    refresh_font_cache();
    Ok(())
}

pub(crate) async fn install_from_zip(path: &Path, opts: ExtractOptions) -> Result<()> {
    let fname = path
        .file_stem()
        .ok_or(FontError::InvalidPath)?
        .to_str()
        .ok_or(FontError::InvalidPath)?;

    let installed = extract_fonts_from_zip(path, fname, opts)?;
    info!("{} ttf or otf fonts installed!", installed);
    refresh_font_cache();
    Ok(())
}

pub(crate) async fn uninstall(name: &str) -> Result<()> {
    remove_font_dir(name)?;
    info!("{} fonts uninstalled!", name);
    refresh_font_cache();
    Ok(())
}

pub(crate) fn refresh_font_cache() {
    info!("Refreshing font cache!");
    if Command::new("fc-cache")
        .args(["-f", "-v"])
        .output()
        .is_err()
    {
        error!("Couldn't refresh font cache! try running \"fc-cache -f -v\"")
    };
}

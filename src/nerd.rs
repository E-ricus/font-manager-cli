use std::{fs::File, path::Path};

use anyhow::Result;
use serde::Deserialize;

use crate::{
    errors::FontError,
    files::{self, ExtractOptions},
    manager,
};

const NERD_URL: &str = "https://github.com/ryanoasis/nerd-fonts/releases/download/v2.1.0/";

#[derive(Deserialize, Debug)]
pub struct NerdFonts {
    fonts: Vec<String>,
}

impl NerdFonts {
    // This is safe to unwrap as in is assured the file exists, otherwise, should fail at
    // compilation
    pub fn new() -> Result<Self> {
        let path = Path::new("assets/nerd.yaml");
        let file = File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }

    pub fn valid_font(&self, font_name: &str) -> Result<()> {
        if !self.fonts.iter().any(|v| v == font_name) {
            return Err(FontError::InvalidNerd.into());
        }
        Ok(())
    }
}

pub(crate) async fn install_nerd(mut font_name: String, mut opts: ExtractOptions) -> Result<()> {
    log::info!("installing {} Nerd Font!", font_name);
    let url = format!("{}{}.zip", NERD_URL, font_name);
    let path = manager::download_zip(&url, "font.zip").await?;
    font_name.push_str("NerdFont");
    opts.delete_zip = true;
    let installed = files::extract_fonts_from_zip(path, &font_name, opts)?;
    manager::manage_installed(installed)
}
pub(crate) async fn uninstall_nerd(mut font_name: String) -> Result<()> {
    log::info!("uninstalling {} Nerd Font!", font_name);
    font_name.push_str("NerdFont");
    files::remove_font_dir(&font_name)?;
    log::info!("{} uninstalled!", font_name);
    manager::refresh_font_cache();
    Ok(())
}

#[cfg(test)]
mod tests_nerd {
    use super::*;

    #[test]
    fn test_new() {
        let nf = NerdFonts::new();
        assert!(nf.is_ok())
    }

    #[test]
    fn test_valid_font() -> Result<()> {
        let nf = NerdFonts::new()?;
        let font_name = "FiraCode".to_string();
        nf.valid_font(&font_name)
    }

    #[test]
    fn test_invalid_font() -> Result<()> {
        let nf = NerdFonts::new()?;
        let font_name = "AnotheOne".to_string();
        let result = nf.valid_font(&font_name);
        assert!(result.is_err());
        Ok(())
    }
}

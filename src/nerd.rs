use std::str::FromStr;
use std::string::ToString;

use anyhow::Result;
use log::info;

use crate::files::{extract_fonts_from_zip, remove_font_dir};
use crate::manager::{download_zip, refresh_font_cache};

const NERD_URL: &str = "https://github.com/ryanoasis/nerd-fonts/releases/download/v2.1.0/";

#[derive(Debug)]
pub(crate) enum NerdFonts {
    FiraCode(String),
    Monoid(String),
    SourceCode(String),
}

impl FromStr for NerdFonts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FiraCode" => Ok(Self::FiraCode(s.to_string())),
            "Monoid" => Ok(Self::Monoid(s.to_string())),
            "SourceCodePro" => Ok(Self::SourceCode(s.to_string())),
            _ => Err("This font doesn't exsist on the nerd aggregator".into()),
        }
    }
}

impl ToString for NerdFonts {
    fn to_string(&self) -> String {
        match self {
            Self::FiraCode(value) => value.to_string(),
            Self::Monoid(value) => value.to_string(),
            Self::SourceCode(value) => value.to_string(),
        }
    }
}

pub(crate) async fn install_nerd(font: NerdFonts) -> Result<()> {
    let mut font_name = font.to_string();
    info!("installing {} Nerd Font!", font_name);
    let url = format!("{}{}.zip", NERD_URL, font_name);
    let path = download_zip(&url, "font.zip").await?;
    font_name.push_str("NerdFont");
    let installed = extract_fonts_from_zip(path, &font_name, true)?;
    info!("{} ttf or otf Nerd fonts installed!", installed);
    refresh_font_cache();
    Ok(())
}
pub(crate) async fn uninstall_nerd(font: NerdFonts) -> Result<()> {
    let mut font_name = font.to_string();
    info!("uninstalling {} Nerd Font!", font_name);
    font_name.push_str("NerdFont");
    remove_font_dir(&font_name)?;
    info!("{} uninstalled!", font_name);
    refresh_font_cache();
    Ok(())
}

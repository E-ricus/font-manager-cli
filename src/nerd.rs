use std::str::FromStr;
use std::string::ToString;

use anyhow::Result;
use log::info;

use crate::files::{extract_fonts_from_zip, remove_font_dir};
use crate::manager::{download_zip, refresh_font_cache};

const NERD_URL: &str = "https://github.com/ryanoasis/nerd-fonts/releases/download/v2.1.0/";

#[derive(Debug)]
pub(crate) enum NerdFonts {
    Number(String),
    Agave(String),
    AnonymousPro(String),
    Arimo(String),
    AurulentSansMono(String),
    BigBlueTerminal(String),
    BitstreamVeraSansMono(String),
    CascadiaCode(String),
    CodeNewRoman(String),
    Cousine(String),
    DaddyTimeMono(String),
    DejaVuSansMono(String),
    DroidSansMono(String),
    FantasqueSansMono(String),
    FiraCode(String),
    FiraMono(String),
    GoMono(String),
    Gohu(String),
    Hack(String),
    Hasklig(String),
    HeavyData(String),
    Hermit(String),
    IAWriter(String),
    IBMPlexMono(String),
    Inconsolata(String),
    InconsolataGo(String),
    InconsolataLGC(String),
    Iosevka(String),
    JetBrainsMono(String),
    Lekton(String),
    LiberationMono(String),
    Meslo(String),
    Monofur(String),
    Monoid(String),
    Mononoki(String),
    MPlus(String),
    Noto(String),
    OpenDyslexic(String),
    Overpass(String),
    ProFont(String),
    ProggyClean(String),
    RobotoMono(String),
    ShareTechMono(String),
    SourceCodePro(String),
    SpaceMono(String),
    Terminus(String),
    Tinos(String),
    Ubuntu(String),
    UbuntuMono(String),
    VictorMono(String),
}

impl FromStr for NerdFonts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "3270" => Ok(Self::Number(s.to_string())),
            "Agave" => Ok(Self::Agave(s.to_string())),
            "AnonymousPro" => Ok(Self::AnonymousPro(s.to_string())),
            "Arimo" => Ok(Self::Arimo(s.to_string())),
            "AurulentSansMono" => Ok(Self::AurulentSansMono(s.to_string())),
            "BigBlueTerminal" => Ok(Self::BigBlueTerminal(s.to_string())),
            "BitstreamVeraSansMono" => Ok(Self::BitstreamVeraSansMono(s.to_string())),
            "CascadiaCode" => Ok(Self::CascadiaCode(s.to_string())),
            "CodeNewRoman" => Ok(Self::CodeNewRoman(s.to_string())),
            "Cousine" => Ok(Self::Cousine(s.to_string())),
            "DaddyTimeMono" => Ok(Self::DaddyTimeMono(s.to_string())),
            "DejaVuSansMono" => Ok(Self::DejaVuSansMono(s.to_string())),
            "DroidSansMono" => Ok(Self::DroidSansMono(s.to_string())),
            "FantasqueSansMono" => Ok(Self::FantasqueSansMono(s.to_string())),
            "FiraCode" => Ok(Self::FiraCode(s.to_string())),
            "FiraMono" => Ok(Self::FiraMono(s.to_string())),
            "Go-Mono" => Ok(Self::GoMono(s.to_string())),
            "Gohu" => Ok(Self::Gohu(s.to_string())),
            "Hack" => Ok(Self::Hack(s.to_string())),
            "Hasklig" => Ok(Self::Hasklig(s.to_string())),
            "HeavyData" => Ok(Self::HeavyData(s.to_string())),
            "Hermit" => Ok(Self::Hermit(s.to_string())),
            "iA-Writer" => Ok(Self::IAWriter(s.to_string())),
            "IBMPlexMono" => Ok(Self::IBMPlexMono(s.to_string())),
            "Inconsolata" => Ok(Self::Inconsolata(s.to_string())),
            "InconsolataGo" => Ok(Self::InconsolataGo(s.to_string())),
            "InconsolataLGC" => Ok(Self::InconsolataLGC(s.to_string())),
            "Iosevka" => Ok(Self::Iosevka(s.to_string())),
            "JetBrainsMono" => Ok(Self::JetBrainsMono(s.to_string())),
            "Lekton" => Ok(Self::Lekton(s.to_string())),
            "LiberationMono" => Ok(Self::LiberationMono(s.to_string())),
            "Meslo" => Ok(Self::Meslo(s.to_string())),
            "Monofur" => Ok(Self::Monofur(s.to_string())),
            "Monoid" => Ok(Self::Monoid(s.to_string())),
            "Mononoki" => Ok(Self::Mononoki(s.to_string())),
            "MPlus" => Ok(Self::MPlus(s.to_string())),
            "Noto" => Ok(Self::Noto(s.to_string())),
            "OpenDyslexic" => Ok(Self::OpenDyslexic(s.to_string())),
            "Overpass" => Ok(Self::Overpass(s.to_string())),
            "ProFont" => Ok(Self::ProFont(s.to_string())),
            "ProggyClean" => Ok(Self::ProggyClean(s.to_string())),
            "RobotoMono" => Ok(Self::RobotoMono(s.to_string())),
            "ShareTechMono" => Ok(Self::ShareTechMono(s.to_string())),
            "SourceCodePro" => Ok(Self::SourceCodePro(s.to_string())),
            "SpaceMono" => Ok(Self::SpaceMono(s.to_string())),
            "Terminus" => Ok(Self::Terminus(s.to_string())),
            "Tinos" => Ok(Self::Tinos(s.to_string())),
            "Ubuntu" => Ok(Self::Ubuntu(s.to_string())),
            "UbuntuMono" => Ok(Self::UbuntuMono(s.to_string())),
            "VictorMono" => Ok(Self::VictorMono(s.to_string())),
            _ => Err("This font doesn't exsist on the nerd aggregator".into()),
        }
    }
}

impl ToString for NerdFonts {
    fn to_string(&self) -> String {
        match self {
            Self::Number(value) => value.to_string(),
            Self::Agave(value) => value.to_string(),
            Self::AnonymousPro(value) => value.to_string(),
            Self::Arimo(value) => value.to_string(),
            Self::AurulentSansMono(value) => value.to_string(),
            Self::BigBlueTerminal(value) => value.to_string(),
            Self::BitstreamVeraSansMono(value) => value.to_string(),
            Self::CascadiaCode(value) => value.to_string(),
            Self::CodeNewRoman(value) => value.to_string(),
            Self::Cousine(value) => value.to_string(),
            Self::DaddyTimeMono(value) => value.to_string(),
            Self::DejaVuSansMono(value) => value.to_string(),
            Self::DroidSansMono(value) => value.to_string(),
            Self::FantasqueSansMono(value) => value.to_string(),
            Self::FiraCode(value) => value.to_string(),
            Self::FiraMono(value) => value.to_string(),
            Self::GoMono(value) => value.to_string(),
            Self::Gohu(value) => value.to_string(),
            Self::Hack(value) => value.to_string(),
            Self::Hasklig(value) => value.to_string(),
            Self::HeavyData(value) => value.to_string(),
            Self::Hermit(value) => value.to_string(),
            Self::IAWriter(value) => value.to_string(),
            Self::IBMPlexMono(value) => value.to_string(),
            Self::Inconsolata(value) => value.to_string(),
            Self::InconsolataGo(value) => value.to_string(),
            Self::InconsolataLGC(value) => value.to_string(),
            Self::Iosevka(value) => value.to_string(),
            Self::JetBrainsMono(value) => value.to_string(),
            Self::Lekton(value) => value.to_string(),
            Self::LiberationMono(value) => value.to_string(),
            Self::Meslo(value) => value.to_string(),
            Self::Monofur(value) => value.to_string(),
            Self::Monoid(value) => value.to_string(),
            Self::Mononoki(value) => value.to_string(),
            Self::MPlus(value) => value.to_string(),
            Self::Noto(value) => value.to_string(),
            Self::OpenDyslexic(value) => value.to_string(),
            Self::Overpass(value) => value.to_string(),
            Self::ProFont(value) => value.to_string(),
            Self::ProggyClean(value) => value.to_string(),
            Self::RobotoMono(value) => value.to_string(),
            Self::ShareTechMono(value) => value.to_string(),
            Self::SourceCodePro(value) => value.to_string(),
            Self::SpaceMono(value) => value.to_string(),
            Self::Terminus(value) => value.to_string(),
            Self::Tinos(value) => value.to_string(),
            Self::Ubuntu(value) => value.to_string(),
            Self::UbuntuMono(value) => value.to_string(),
            Self::VictorMono(value) => value.to_string(),
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

use anyhow::Result;

use crate::files::{self, ExtractOptions};
use crate::manager;

const NERD_URL: &str = "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/";
//TODO: Read from file?
pub const VALID_FONTS: [&str; 50] = [
    "3270",
    "Agave",
    "AnonymousPro",
    "Arimo",
    "AurulentSansMono",
    "BigBlueTerminal",
    "BitstreamVeraSansMono",
    "CascadiaCode",
    "CodeNewRoman",
    "Cousine",
    "DaddyTimeMono",
    "DejaVuSansMono",
    "DroidSansMono",
    "FantasqueSansMono",
    "FiraCode",
    "FiraMono",
    "Go-Mono",
    "Gohu",
    "Hack",
    "Hasklig",
    "HeavyData",
    "Hermit",
    "iA-Writer",
    "IBMPlexMono",
    "Inconsolata",
    "InconsolataGo",
    "InconsolataLGC",
    "Iosevka",
    "JetBrainsMono",
    "Lekton",
    "LiberationMono",
    "Meslo",
    "Monofur",
    "Monoid",
    "Mononoki",
    "MPlus",
    "Noto",
    "OpenDyslexic",
    "Overpass",
    "ProFont",
    "ProggyClean",
    "RobotoMono",
    "ShareTechMono",
    "SourceCodePro",
    "SpaceMono",
    "Terminus",
    "Tinos",
    "Ubuntu",
    "UbuntuMono",
    "VictorMono",
];

pub(crate) async fn install_nerd(font: &str, mut opts: ExtractOptions) -> Result<()> {
    log::info!("installing {} Nerd Font!", font);
    let url = format!("{}{}.zip", NERD_URL, font);
    let path = manager::download_zip(&url, "font.zip").await?;
    opts.delete_zip = true;
    let installed = files::extract_fonts_from_zip(path, font, opts)?;
    manager::manage_installed(installed)
}
pub(crate) async fn uninstall_nerd(font: &str) -> Result<()> {
    log::info!("uninstalling {} Nerd Font!", font);
    files::remove_font_dir(font)?;
    log::info!("{} uninstalled!", font);
    manager::refresh_font_cache();
    Ok(())
}

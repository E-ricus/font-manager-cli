mod nerd;

use crate::nerd::{install_nerd, NerdFonts};
use std::path::PathBuf;

use log::{error, info};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Font manager", about = "a simple font manager utility")]
enum FontManager {
    /// Install the given font
    Install(Install),
    /// Uninstall the given font name if is already installed
    Uninstall(Uninstall),
}

#[derive(Debug, StructOpt)]
struct Install {
    #[structopt(short = "n", long = "nerd")]
    /// Indicates if it should grab the font from the nerd aggregator
    /// If this is send, the nerd_name should be given as well
    nerd: bool,
    /// Nerd font name to be installed, only used if nerd is setted to true
    /// The name should be the same one as on the font aggregator project
    nerd_name: Option<NerdFonts>,
    #[structopt(short = "f", long = "from-file")]
    path: Option<PathBuf>,
    #[structopt(short = "u", long = "from-url")]
    url: Option<String>,
}

impl Install {
    fn valid_command(&self) -> Result<(), String> {
        let valid = vec![self.nerd, self.path.is_some(), self.url.is_some()];
        let mut count = 0;
        for v in valid {
            if count > 1 {
                return Err(
                    "--nerd, --from-file and --from-url are mutuable exclusive, send only one"
                        .into(),
                );
            }
            if v {
                count += 1;
            }
        }
        if self.nerd && self.nerd_name.is_none() {
            return Err("nerd_name must be sent if --nerd is selected".into());
        }
        if !self.nerd && self.nerd_name.is_some() {
            return Err("nerd_name is not valid if --nerd is not selected".into());
        }
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
struct Uninstall {
    /// Font name to be uninstalled
    /// For not nerd fonts, you'll need to give the name as it is on the .ttf files
    nerd_name: String,
}

fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    pretty_env_logger::init();
    let opt = FontManager::from_args();
    println!("{:#?}", opt);
    if let FontManager::Install(install) = opt {
        if let Err(error) = install.valid_command() {
            error!("invalid command: {}", error);
            std::process::exit(0);
        }
        if install.nerd {
            // Safe to unwrap, as this is already validated
            install_nerd(install.nerd_name.unwrap())
        }
        info!("installing font!");
    }
}

#[cfg(test)]
mod command {
    use super::*;

    #[test]
    fn test_valid_install_command() {
        let install = Install {
            nerd: true,
            nerd_name: Some(NerdFonts::SourceCode("SourceCode".into())),
            path: None,
            url: None,
        };
        assert!(install.valid_command().is_ok())
    }

    #[test]
    fn test_invalid_install_command() {
        let mut install = Install {
            nerd: true,
            nerd_name: Some(NerdFonts::SourceCode("SourceCode".into())),
            path: Some("the path".into()),
            url: None,
        };
        assert!(install.valid_command().is_err());
        install.url = Some("the url".into());
        assert!(install.valid_command().is_err());
    }

    #[test]
    fn test_install_invalid_nerd() {
        let install = Install {
            nerd: true,
            nerd_name: None,
            path: None,
            url: None,
        };
        assert!(install.valid_command().is_err());
        let install = Install {
            nerd: false,
            nerd_name: Some(NerdFonts::SourceCode("SourceCode".into())),
            path: None,
            url: None,
        };
        assert!(install.valid_command().is_err());
    }
}

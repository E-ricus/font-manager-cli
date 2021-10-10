mod errors;
mod files;
mod manager;
mod nerd;

use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use log::debug;
use structopt::StructOpt;

use crate::errors::FontError;
use crate::manager::{install_from_url, install_from_zip, uninstall};
use crate::nerd::{install_nerd, uninstall_nerd, NerdFonts};

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
    /// --nerd option is mutually exclusive with --from-zip and --from-url
    nerd: bool,
    /// Nerd font name to be installed, only used if nerd is setted to true
    /// The name should be the same one as on the font aggregator project
    nerd_name: Option<NerdFonts>,
    #[structopt(short = "z", long = "from-zip")]
    /// Path to the location of the zip file with the fonts to be installed
    /// --from-zip option is mutually exclusive with --nerd  and --from-url
    path: Option<PathBuf>,
    #[structopt(short = "u", long = "from-url")]
    /// url that downloads a zip with the font
    /// --from-url option is mutually exclusive with --nerd  and --from-zip
    url: Option<String>,
    /// indicates if the .zip file with the fonts should be removed
    /// for --nerd it will always delete the zip even if this is provided
    #[structopt(short = "d", long = "delete-zip")]
    delete_zip: bool,
}

impl Install {
    fn valid_command(&self) -> Result<()> {
        let valid = vec![self.nerd, self.path.is_some(), self.url.is_some()];
        let mut count = 0;
        for v in valid {
            if count > 1 {
                return Err(FontError::CommandError(String::from(
                    "--nerd, --from-zip and --from-url are mutuable exclusive, send only one",
                ))
                .into());
            }
            if v {
                count += 1;
            }
        }
        if count != 1 {
            return Err(FontError::CommandError(String::from(
                "at least one and just one option must be provided: --nerd, --from-zip and --from-url",
            ))
            .into());
        }
        if self.nerd && self.nerd_name.is_none() {
            return Err(FontError::CommandError(String::from(
                "nerd_name must be sent if --nerd is selected",
            ))
            .into());
        }
        if !self.nerd && self.nerd_name.is_some() {
            return Err(FontError::CommandError(String::from(
                "nerd_name is not valid if --nerd is not selected",
            ))
            .into());
        }
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
struct Uninstall {
    /// Font name to be uninstalled
    /// For not nerd fonts, you'll need to give the dir_name as it is on the .fonts/ directory
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let log = env::var("RUST_LOG");
    if log.is_err() {
        env::set_var("RUST_LOG", "INFO");
    }
    if let Ok(v) = log {
        if v.is_empty() {
            env::set_var("RUST_LOG", "INFO");
        }
    }

    pretty_env_logger::init();
    let opt = FontManager::from_args();
    return manage_font(opt).await;
}

async fn manage_font(opt: FontManager) -> Result<()> {
    debug!("{:#?}", opt);
    match opt {
        FontManager::Install(i) => {
            i.valid_command()?;
            if i.nerd {
                // Safe to unwrap, as this is already validated
                return install_nerd(i.nerd_name.unwrap()).await;
            }
            if let Some(url) = i.url {
                return install_from_url(&url, i.delete_zip).await;
            }
            if let Some(path) = i.path {
                return install_from_zip(&path, i.delete_zip).await;
            }
        }
        FontManager::Uninstall(u) => match NerdFonts::from_str(&u.name) {
            Ok(n) => uninstall_nerd(n).await?,
            Err(_) => uninstall(&u.name).await?,
        },
    }
    Ok(())
}

#[cfg(test)]
mod test_command {
    use super::*;

    #[test]
    fn test_valid_install_command() {
        let install = Install {
            nerd: true,
            nerd_name: Some(NerdFonts::SourceCode("SourceCode".into())),
            path: None,
            url: None,
            delete_zip: true,
        };
        assert!(install.valid_command().is_ok())
    }

    #[test]
    fn test_invalid_install_command() {
        // Given more than one flag should fail
        let mut install = Install {
            nerd: true,
            nerd_name: Some(NerdFonts::SourceCode("SourceCode".into())),
            path: Some("the path".into()),
            url: None,
            delete_zip: true,
        };
        assert!(install.valid_command().is_err());

        // Given all flags, should fail
        install.url = Some("the url".into());
        assert!(install.valid_command().is_err());

        // Given no flags, should fail
        let install = Install {
            nerd: false,
            nerd_name: None,
            path: None,
            url: None,
            delete_zip: true,
        };

        // Given any flag and last, should fail
        assert!(install.valid_command().is_err());
        let install = Install {
            nerd: true,
            nerd_name: Some(NerdFonts::SourceCode("SourceCode".into())),
            path: None,
            url: Some("the url".into()),
            delete_zip: true,
        };
        assert!(install.valid_command().is_err());
    }

    #[test]
    fn test_install_invalid_nerd() {
        // Given no nerd name with nerd flag, should fail
        let install = Install {
            nerd: true,
            nerd_name: None,
            path: None,
            url: None,
            delete_zip: true,
        };
        assert!(install.valid_command().is_err());

        // Given a nerd name without a nerd flag, should fail
        let install = Install {
            nerd: false,
            nerd_name: Some(NerdFonts::SourceCode("SourceCode".into())),
            path: None,
            url: None,
            delete_zip: true,
        };
        assert!(install.valid_command().is_err());
    }
}

#[cfg(test)]
mod tests_manager {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_nerd_monoid() {
        env::set_var("RUST_LOG", "DEBUG");
        pretty_env_logger::init();
        let install = Install {
            nerd: true,
            nerd_name: Some(NerdFonts::SourceCode("Monoid".into())),
            path: None,
            url: None,
            delete_zip: true,
        };
        let opt = FontManager::Install(install);
        let result = manage_font(opt).await;
        assert!(result.is_ok());

        let uninstall = Uninstall {
            name: String::from("MonoidNerdFont"),
        };
        let opt = FontManager::Uninstall(uninstall);
        let result = manage_font(opt).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_zip_firacode() {
        env::set_var("RUST_LOG", "DEBUG");
        pretty_env_logger::init();
        let install = Install {
            nerd: false,
            nerd_name: None,
            path: Some("test-data/FiraCodeTest.zip".into()),
            url: None,
            delete_zip: false,
        };
        let opt = FontManager::Install(install);
        let result = manage_font(opt).await;
        assert!(result.is_ok());

        let uninstall = Uninstall {
            name: String::from("FiraCodeTest"),
        };
        let opt = FontManager::Uninstall(uninstall);
        let result = manage_font(opt).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_url_jetbrains() {
        env::set_var("RUST_LOG", "DEBUG");
        pretty_env_logger::init();
        let install = Install {
            nerd: false,
            nerd_name: None,
            path: None,
            url: Some("https://download.jetbrains.com/fonts/JetBrainsMono-2.242.zip".into()),
            delete_zip: false,
        };
        let opt = FontManager::Install(install);
        let result = manage_font(opt).await;
        assert!(result.is_ok());

        let uninstall = Uninstall {
            name: String::from("JetBrainsMono-2.242"),
        };
        let opt = FontManager::Uninstall(uninstall);
        let result = manage_font(opt).await;
        assert!(result.is_ok());
    }
}

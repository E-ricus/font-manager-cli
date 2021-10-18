mod command;
mod errors;
mod files;
mod manager;
mod nerd;

use std::env;

use anyhow::Result;
use structopt::StructOpt;

use crate::command::FontManager;
use crate::files::ExtractOptions;
use crate::nerd::NerdFonts;

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
    log::debug!("Args command: {:#?}", opt);
    let nf = NerdFonts::new()?;
    match opt {
        FontManager::Install(i) => {
            i.valid_command()?;

            let ext_opt = ExtractOptions {
                delete_zip: i.delete_zip,
                use_otf: i.use_otf,
                interactive: i.interactive,
            };

            if i.nerd {
                // Safe to unwrap, as this is already validated
                let font_name = i.nerd_name.unwrap();
                nf.valid_font(&font_name)?;

                return nerd::install_nerd(font_name, ext_opt).await;
            }
            if let Some(url) = i.url {
                return manager::install_from_url(&url, ext_opt).await;
            }
            if let Some(path) = i.path {
                return manager::install_from_zip(&path, ext_opt).await;
            }
        }
        FontManager::Uninstall(u) => match nf.valid_font(&u.name) {
            Ok(_) => nerd::uninstall_nerd(u.name).await?,
            Err(_) => manager::uninstall(&u.name).await?,
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests_manager {
    use super::*;
    use crate::command::{Install, Uninstall};

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_nerd_monoid() {
        env::set_var("RUST_LOG", "DEBUG");
        pretty_env_logger::init();
        let mut install = Install::new();
        install.nerd = true;
        install.nerd_name = Some("Monoid".into());
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
        let mut install = Install::new();
        install.path = Some("test-data/FiraCodeTest.zip".into());
        install.delete_zip = false;
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
        let mut install = Install::new();
        install.url = Some("https://download.jetbrains.com/fonts/JetBrainsMono-2.242.zip".into());
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

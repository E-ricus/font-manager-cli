use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::errors::FontError;

#[derive(Debug, Parser)]
#[command(name = "Font manager", about = "a simple font manager utility")]
pub(super) enum FontManager {
    /// Install the given font
    Install(Install),
    /// Uninstall the given font name if is already installed
    Uninstall(Uninstall),
}

#[derive(Debug, Parser)]
pub(super) struct Install {
    /// Indicates if it should grab the font from the nerd aggregator
    /// --nerd option is mutually exclusive with --from-zip and --from-url
    #[arg(short = 'n', long = "nerd")]
    pub(super) nerd: Option<String>,
    /// Path to the location of the zip file with the fonts to be installed
    /// --from-zip option is mutually exclusive with --nerd  and --from-url
    #[arg(short = 'z', long = "from-zip")]
    pub(super) path: Option<PathBuf>,
    /// url that downloads a zip with the font
    /// --from-url option is mutually exclusive with --nerd  and --from-zip
    #[arg(short = 'u', long = "from-url")]
    pub(super) url: Option<String>,
    /// indicates if the .zip file with the fonts should be removed
    /// for --nerd it will always delete the zip even if this is provided
    #[arg(short = 'd', long = "delete-zip")]
    pub(super) delete_zip: bool,
    /// indicates if should ignore .ttf and use .otf version
    #[arg(long = "use-otf")]
    pub(super) use_otf: bool,
    /// indicates if user shold accept each file to be installed
    #[arg(short = 'i', long = "interactive")]
    pub(super) interactive: bool,
}

impl Install {
    #[cfg(test)]
    pub(super) fn new() -> Self {
        Self {
            nerd: None,
            path: None,
            url: None,
            delete_zip: true,
            use_otf: false,
            interactive: false,
        }
    }
    pub(super) fn valid_command(&self) -> Result<()> {
        let valid = [self.nerd.is_some(), self.path.is_some(), self.url.is_some()];
        if valid.into_iter().filter(|v| *v).count() != 1 {
            return Err(FontError::CommandError(String::from(
                "one and only one option must be provided: --nerd, --from-zip and --from-url",
            ))
            .into());
        }
        if let Some(nerd_name) = &self.nerd {
            if !crate::nerd::VALID_FONTS.contains(&nerd_name.as_str()) {
                return Err(FontError::CommandError(String::from("Nerd font not valid")).into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub(super) struct Uninstall {
    /// Font name to be uninstalled
    /// For not nerd fonts, you'll need to give the dir_name as it is on the .fonts/ directory
    pub(super) name: String,
}

#[cfg(test)]
mod test_command {
    use super::*;

    #[test]
    fn test_valid_install_command() {
        let install = Install {
            nerd: Some("SourceCodePro".to_string()),
            path: None,
            url: None,
            delete_zip: true,
            use_otf: false,
            interactive: false,
        };
        assert!(install.valid_command().is_ok())
    }

    #[test]
    fn test_invalid_install_command() {
        // Given more than one flag should fail
        let mut install = Install::new();
        install.nerd = Some("SourceCodePro".into());
        install.path = Some("the path".into());
        assert!(install.valid_command().is_err());

        // Given all flags, should fail
        install.url = Some("the url".into());
        assert!(install.valid_command().is_err());

        // Given no flags, should fail
        let install = Install::new();
        assert!(install.valid_command().is_err());

        // Given any flag and last, should fail
        let mut install = Install::new();
        install.nerd = Some("SourceCodePro".into());
        install.url = Some("the url".into());
        assert!(install.valid_command().is_err());
    }

    #[test]
    fn test_install_invalid_nerd() {
        // Given a invalid nerd name, should fail
        let mut install = Install::new();
        install.nerd = Some("Sourcecode".into());
        assert!(install.valid_command().is_err());
    }
}

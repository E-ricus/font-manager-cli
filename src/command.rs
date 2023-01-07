use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::errors::FontError;

#[derive(Debug, Parser)]
#[clap(name = "Font manager", about = "a simple font manager utility")]
pub(super) enum FontManager {
    /// Install the given font
    Install(Install),
    /// Uninstall the given font name if is already installed
    Uninstall(Uninstall),
}

#[derive(Debug, Parser)]
pub(super) struct Install {
    #[clap(short = 'n', long = "nerd")]
    /// Indicates if it should grab the font from the nerd aggregator
    /// If this is send, the nerd_name should be given as well
    /// --nerd option is mutually exclusive with --from-zip and --from-url
    pub(super) nerd: bool,
    /// Nerd font name to be installed, only used if nerd is setted to true
    /// The name should be the same one as on the font aggregator project
    pub(super) nerd_name: Option<String>,
    #[clap(short = 'z', long = "from-zip")]
    /// Path to the location of the zip file with the fonts to be installed
    /// --from-zip option is mutually exclusive with --nerd  and --from-url
    pub(super) path: Option<PathBuf>,
    #[clap(short = 'u', long = "from-url")]
    /// url that downloads a zip with the font
    /// --from-url option is mutually exclusive with --nerd  and --from-zip
    pub(super) url: Option<String>,
    /// indicates if the .zip file with the fonts should be removed
    /// for --nerd it will always delete the zip even if this is provided
    #[clap(short = 'd', long = "delete-zip")]
    pub(super) delete_zip: bool,
    /// indicates if should ignore .ttf and use .otf version
    #[clap(long = "use-otf")]
    pub(super) use_otf: bool,
    /// indicates if user shold accept each file to be installed
    #[clap(short = 'i', long = "interactive")]
    pub(super) interactive: bool,
}

impl Install {
    #[cfg(test)]
    pub(super) fn new() -> Self {
        Self {
            nerd: false,
            nerd_name: None,
            path: None,
            url: None,
            delete_zip: true,
            use_otf: false,
            interactive: false,
        }
    }
    pub(super) fn valid_command(&self) -> Result<()> {
        let valid = vec![self.nerd, self.path.is_some(), self.url.is_some()];
        let flags = valid.into_iter().filter(|v| *v).count();
        if flags != 1 {
            return Err(FontError::CommandError(String::from(
                "one and only one option must be provided: --nerd, --from-zip and --from-url",
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
        if self.nerd && self.nerd_name.is_some() {
            let name: &str = &self.nerd_name.clone().unwrap();
            if !crate::nerd::VALID_FONTS.contains(&name) {
                return Err(FontError::CommandError(String::from(
                    "nerd_name is not valid if --nerd is not selected",
                ))
                .into());
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
            nerd: true,
            nerd_name: Some("SourceCodePro".to_string()),
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
        install.nerd = true;
        install.nerd_name = Some("SourceCodePro".into());
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
        install.nerd = true;
        install.nerd_name = Some("SourceCodePro".into());
        install.url = Some("the url".into());
        assert!(install.valid_command().is_err());
    }

    #[test]
    fn test_install_invalid_nerd() {
        // Given no nerd name with nerd flag, should fail
        let mut install = Install::new();
        install.nerd = true;
        assert!(install.valid_command().is_err());

        // Given a nerd name without a nerd flag, should fail
        let mut install = Install::new();
        install.nerd_name = Some("SourceCodePro".into());
        assert!(install.valid_command().is_err());
        // Given a invalid nerd name, should fail
        let mut install = Install::new();
        install.nerd = true;
        install.nerd_name = Some("Sourcecode".into());
        assert!(install.valid_command().is_err());
    }
}

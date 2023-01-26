use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum FontError {
    #[error("invalid comand: `{0}`")]
    CommandError(String),
    #[error("home folder not found, $HOME might not be set")]
    HomeNotFound,
    #[error("the provided path doesn't exist or is damaged")]
    InvalidPath,
    #[error("no fonts were installed! check if the files were ignored")]
    FontsIgnored,
}

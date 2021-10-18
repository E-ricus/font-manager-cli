use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum FontError {
    #[error("invalid comand: `{0}`")]
    CommandError(String),
    #[error("home folder not found, $HOME might not be set")]
    HomeNotFound,
    #[error("the provided path doesn't exsist or is damaged")]
    InvalidPath,
    #[error("no fonts were installed! check if the files were ignored")]
    FontsIgnored,
    #[error("there's no a matching font with that name on the nerd iconinc aggregator")]
    InvalidNerd,
    // TODO: Maybe transform io and files error to more user friendly error
    // #[error("something went wrong! please report an issue")]
    // UnexpectedErorr,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum FontError {
    #[error("invalid comand: `{0}`")]
    CommandError(String),
    #[error("home folder not found, $HOME might not be set")]
    HomeNotFound,
    #[error("the provided path doesn't exsist or is damaged")]
    InvalidPath,
    // TODO: Maybe transform io and files error to more user friendly error
    // #[error("something went wrong! please report an issue")]
    // UnexpectedErorr,
}

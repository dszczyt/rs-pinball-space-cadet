use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("Bad handle")]
    BadHandle,

    #[error("No type field")]
    NoTypeFile,
}

pub struct Loader {}

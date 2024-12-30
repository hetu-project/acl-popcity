use error_macros::ErrorWithCode;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, ErrorWithCode)]
pub enum Error {
    #[error("No operator config found at this path: {0}")]
    #[code(404)]
    ConfigMissing(PathBuf),

    #[error("Unknown error occurred.")]
    #[code(500)]
    UnknownError,

    #[error("Everything is fine.")]
    #[code(200)]
    Success,
}

fn main() {
    let err1 = Error::ConfigMissing(PathBuf::from("/path/to/config"));
    let err2 = Error::UnknownError;
    let err3 = Error::Success;

    println!(
        "Error: {}, Code: {}",
        err1.error_message(),
        err1.error_code()
    );
    println!(
        "Error: {}, Code: {}",
        err2.error_message(),
        err2.error_code()
    );
    println!(
        "Error: {}, Code: {}",
        err3.error_message(),
        err3.error_code()
    );
}

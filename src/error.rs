use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid email")]
    InvalidEmail,
}

pub type Result<T> = std::result::Result<T, Error>;

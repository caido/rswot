use thiserror::Error;

/// Error type for validation
#[derive(Error, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    #[error("Invalid email")]
    InvalidEmail,

    #[error("Email is from a known abusive domain")]
    Abuse,

    #[error("Email is marked as stop")]
    Stop,

    #[error("Email is not academic")]
    NotAcademic,
}

pub type Result<T> = std::result::Result<T, Error>;

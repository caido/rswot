use crate::{Email, Tld};

/// Status of validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// Email is a valid academic email
    Valid,
    /// Email is from a known abusive domain
    Abuse,
    /// Email is marked as stop
    Stop,
}

/// Result of validation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Validation {
    /// Email address that was validated
    pub email: Email,
    /// Top-level domain of email
    pub tld: Tld,
    /// Names of the institution that the email is from
    pub institution_names: Option<Vec<String>>,
    /// Status of validation
    pub status: Status,
}

impl Validation {
    pub fn is_academic(&self) -> bool {
        self.status == Status::Valid
    }
}

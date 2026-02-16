use crate::{Email, Tld};

/// Result of validation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Validation {
    /// Email address that was validated
    pub email: Email,
    /// Top-level domain of email
    pub tld: Tld,
    /// Names of the institution that the email is from
    pub institution_names: Option<Vec<String>>,
}

impl Validation {
    pub(crate) fn new(email: Email) -> Validation {
        let tld = email.tld();
        Validation {
            email,
            tld,
            institution_names: None,
        }
    }

    pub(crate) fn with_institutions(self, institution_names: Vec<String>) -> Validation {
        Validation {
            institution_names: Some(institution_names),
            ..self
        }
    }
}

// Logic of this parsing was taken from the validator crate
// https://github.com/Keats/validator/blob/cf51a327390de7f2abbab7a024b431784ce0be6c/validator/src/validation/email.rs

use std::fmt;
use std::sync::LazyLock;

use regex::Regex;

use crate::{Error, Result, Tld};

static EMAIL_USER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap());
static EMAIL_DOMAIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    user: String,
    domain: String,
}

impl Email {
    pub fn parse(email: &str) -> Result<Email> {
        if email.is_empty() || !email.contains('@') {
            return Err(Error::InvalidEmail);
        }

        let parts: Vec<&str> = email.rsplitn(2, '@').collect();
        let user_part = parts[1];
        let domain_part = parts[0];

        // validate the length of each part of the email, BEFORE doing the regex
        // according to RFC5321 the max length of the local part is 64 characters
        // and the max length of the domain part is 255 characters
        // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
        if user_part.chars().count() > 64 || domain_part.chars().count() > 255 {
            return Err(Error::InvalidEmail);
        }

        if !EMAIL_USER.is_match(user_part) {
            return Err(Error::InvalidEmail);
        }

        if !EMAIL_DOMAIN.is_match(domain_part) {
            return Err(Error::InvalidEmail);
        }

        Ok(Email {
            user: user_part.to_string(),
            domain: domain_part.to_string(),
        })
    }

    pub(crate) fn tld(&self) -> Tld {
        Tld::new_owned(self.domain_parts()[0].to_string())
    }

    pub(crate) fn domain_parts(&self) -> Vec<&str> {
        self.domain.rsplit('.').collect()
    }
}

impl TryFrom<String> for Email {
    type Error = Error;
    fn try_from(value: String) -> Result<Email> {
        Email::parse(&value)
    }
}

impl TryFrom<&str> for Email {
    type Error = Error;
    fn try_from(value: &str) -> Result<Email> {
        Email::parse(value)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_email() {
        let email = Email::parse("orhanbalci@ku.edu.tr").unwrap();
        assert_eq!(email.domain, "ku.edu.tr");
        assert_eq!(email.user, "orhanbalci");
        assert_eq!(email.domain_parts(), vec!["tr", "edu", "ku"]);
        assert_eq!(email.tld(), Tld::new("tr"));
    }
}

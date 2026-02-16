use self::data::{get_institution_names, is_abused, is_stop_list, is_under_tld};
pub use self::email::Email;
pub use self::error::{Error, Result};
pub use self::tld::Tld;
pub use self::validation::Validation;

mod data;
mod email;
mod error;
mod tld;
mod validation;

pub fn validate<T>(email: T) -> Result<Validation>
where
    T: TryInto<Email>,
    T::Error: Into<Error>,
{
    let email = email.try_into()?;
    let domain_parts = email.domain_parts();

    if is_stop_list(&domain_parts) {
        return Err(Error::Stop);
    }

    if is_abused(&domain_parts) {
        return Err(Error::Abuse);
    }

    if is_under_tld(&domain_parts) {
        return Ok(Validation::new(email));
    }

    if let Some(institution_names) = get_institution_names(&domain_parts) {
        return Ok(Validation::new(email).with_institutions(institution_names));
    }

    return Err(Error::NotAcademic);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let tests = Vec::from([
            ("lreilly@stanford.edu", None),
            ("LREILLY@STANFORD.EDU", None),
            ("Lreilly@Stanford.Edu", None),
            ("lreilly@slac.stanford.edu", None),
            ("lreilly@strath.ac.uk", None),
            ("lreilly@soft-eng.strath.ac.uk", None),
            ("lee@ugr.es", None),
            ("lee@uottawa.ca", None),
            ("lee@ucy.ac.cy", None),
            ("lee@leerilly.net", Some(Error::NotAcademic)),
            ("lee@gmail.com", Some(Error::NotAcademic)),
            ("lee@stanford.edu.com", Some(Error::NotAcademic)),
            ("lee@strath.ac.uk.com", Some(Error::NotAcademic)),
            ("john@stanford.edu", None),
            ("john@slac.stanford.edu", None),
            ("john@www.stanford.edu", None),
            ("user@gmail.com", Some(Error::NotAcademic)),
            ("", Some(Error::InvalidEmail)),
            ("the", Some(Error::InvalidEmail)),
            (" stanford.edu", Some(Error::InvalidEmail)),
            (".com", Some(Error::InvalidEmail)),
            ("lee@strath.ac.uk ", None),
            (" gmail.com", Some(Error::NotAcademic)),
            ("lee@stud.uni-corvinus.hu", None),
            ("lee@harvard.edu", None),
            ("lee@mail.harvard.edu", None),
            ("imposter@si.edu", Some(Error::NotAcademic)),
            ("lee@acmt.ac.ir", None),
            ("lee@australia.edu", Some(Error::NotAcademic)),
            ("john@si.edu", Some(Error::NotAcademic)),
            ("john@foo.si.edu", Some(Error::NotAcademic)),
            ("john@america.edu", Some(Error::NotAcademic)),
            ("john@folger.edu", Some(Error::NotAcademic)),
            ("foo@bar.invalid", Some(Error::NotAcademic)),
            ("emile@usherbrooke.ca", None),
        ]);

        for (email, result) in tests {
            println!("testing {}", email);

            let validation = validate(email);
            if let Some(error) = result {
                assert_eq!(validation, Err(error));
            } else {
                assert!(validation.is_ok());
            }
        }
    }
}

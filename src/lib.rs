use rust_embed::RustEmbed;
use url::{ParseError, Url};
use validator::{Validate, ValidateEmail, ValidateUrl};

pub use self::email::Email;
pub use self::error::{Error, Result};
pub use self::tld::Tld;
pub use self::validation::{Status, ValidationResult};

mod data;
mod email;
mod error;
mod tld;
mod validation;

include!(concat!(env!("OUT_DIR"), "/tlds.rs"));
include!(concat!(env!("OUT_DIR"), "/stoplist.rs"));

#[derive(RustEmbed)]
#[folder = "domains/"]
struct Domains;

pub fn is_academic<T: AsRef<str>>(email: T) -> bool {
    get_domain(email.as_ref())
        .map(|d| get_domain_parts(d))
        .map(|dp| {
            let is_stop_listed = is_stop_listed(&dp);
            let is_under_tld = is_under_tld(&dp);
            return !is_stop_listed
                && (is_under_tld
                    || get_school_names(email)
                        .map(|sn| !sn.is_empty())
                        .unwrap_or(false));
        })
        .unwrap_or(false)
}

pub fn get_school_names<T: AsRef<str>>(domain: T) -> Option<Vec<String>> {
    get_domain(domain.as_ref())
        .map(|dom| get_domain_parts(dom))
        .and_then(|dp| get_institution_name(dp))
}

pub fn is_under_tld<T: AsRef<str>>(parts: &[T]) -> bool {
    check_set(
        &TLDS,
        &parts.iter().map(|s| s.as_ref()).collect::<Vec<&str>>()[..],
    )
}

pub fn is_stop_listed<T: AsRef<str>>(parts: &[T]) -> bool {
    check_set(
        &STOPLIST,
        &parts.iter().map(|s| s.as_ref()).collect::<Vec<&str>>()[..],
    )
}

fn get_institution_name(domain_parts: Vec<String>) -> Option<Vec<String>> {
    let mut domain_path = domain_parts[0].clone();

    for part in domain_parts.iter().skip(1) {
        domain_path.push_str("/");
        domain_path.push_str(part);
        let path = format!("{}.txt", domain_path);
        let domain_file_content = Domains::get(&path).and_then(|text| {
            std::str::from_utf8(text.data.as_ref())
                .ok()
                .map(|a| a.to_string())
        });
        if let Some(fc) = domain_file_content {
            return Some(
                fc.split("\n")
                    .filter(|institution_name| !institution_name.is_empty())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        } else {
            continue;
        }
    }

    return None;
}

fn check_set(set: &phf::Set<&str>, parts: &[&str]) -> bool {
    let mut needle = String::new();
    for &part in parts {
        needle = format!("{}{}", part, needle);
        if set.contains(&needle) {
            return true;
        } else {
            needle = format!(".{}", needle);
        }
    }
    return false;
}

fn get_domain(address: &str) -> Option<String> {
    if address.validate_email() {
        address
            .trim()
            .to_lowercase()
            .split_once('@')
            .map(|a| a.1.to_string())
    } else {
        let url = Url::parse(address.trim());
        match url {
            Ok(u) => u.host_str().map(|a| a.to_string()),
            Err(ParseError::RelativeUrlWithoutBase) => {
                let updated_url = Url::parse(&format!("http://{}", address.trim()));
                if let Ok(uu) = updated_url {
                    uu.host_str().map(|a| a.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn get_domain_parts(domain: String) -> Vec<String> {
    domain.rsplit('.').map(|a| a.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn read_domain_file() {
        match super::Domains::get("es/ugr.txt") {
            Some(text) => println!("{:?}", std::str::from_utf8(text.data.as_ref())),
            None => println!("File not found"),
        }
        assert!(true);
    }

    #[test]
    fn is_under_tld() {
        assert!(!super::is_under_tld(&["es", "ugr"]));
    }

    #[test]
    fn is_stop_listed() {
        assert!(!super::is_stop_listed(&["es", "ugr"]));
    }

    #[test]
    fn get_domain() {
        //let result = ["tr", "edu", "ku"];
        let result = Some("ku.edu.tr".to_string());
        assert_eq!(result, super::get_domain("orhanbalci@ku.edu.tr"));
    }

    #[test]
    fn get_domain_parts() {
        let result = vec!["tr", "edu", "ku"];
        assert_eq!(
            result,
            super::get_domain_parts(super::get_domain("orhanbalci@ku.edu.tr").unwrap())
        );
    }

    #[test]
    fn get_university_name() {
        let result = vec!["Koç Üniversitesi", "Koç University"];
        assert_eq!(
            result,
            super::get_school_names("orhanbalci@ku.edu.tr".to_string())
                .expect("University name should be on text")
        );
    }

    #[test]
    fn is_academic() {
        assert!(super::is_academic("orhanbalci@ku.edu.tr"))
    }

    #[test]
    fn find_school_names() {
        assert_eq!(None, super::get_school_names("folger.edu".to_string()))
    }

    #[test]
    fn is_academic_full() {
        let tests = HashMap::from([
            ("lreilly@stanford.edu", true),
            ("LREILLY@STANFORD.EDU", true),
            ("Lreilly@Stanford.Edu", true),
            ("lreilly@slac.stanford.edu", true),
            ("lreilly@strath.ac.uk", true),
            ("lreilly@soft-eng.strath.ac.uk", true),
            ("lee@ugr.es", true),
            ("lee@uottawa.ca", true),
            //("lee@mother.edu.ru", true),
            ("lee@ucy.ac.cy", true),
            ("lee@leerilly.net", false),
            ("lee@gmail.com", false),
            ("lee@stanford.edu.com", false),
            ("lee@strath.ac.uk.com", false),
            ("stanford.edu", true),
            ("slac.stanford.edu", true),
            ("www.stanford.edu", true),
            ("http://www.stanford.edu", true),
            ("http://www.stanford.edu:9393", true),
            ("strath.ac.uk", true),
            ("soft-eng.strath.ac.uk", true),
            ("ugr.es", true),
            ("uottawa.ca", true),
            //("mother.edu.ru", true),
            ("ucy.ac.cy", true),
            ("leerilly.net", false),
            ("gmail.com", false),
            ("stanford.edu.com", false),
            ("strath.ac.uk.com", false),
            ("", false),
            ("the", false),
            (" stanford.edu", true),
            ("lee@strath.ac.uk ", true),
            (" gmail.com", false),
            ("lee@stud.uni-corvinus.hu", true),
            ("lee@harvard.edu", true),
            ("lee@mail.harvard.edu", true),
            ("imposter@si.edu", false),
            ("lee@acmt.ac.ir", true),
            ("lee@australia.edu", false),
            ("si.edu", false),
            ("foo.si.edu", false),
            ("america.edu", false),
            ("folger.edu", false),
            ("foo@bar.invalid", false),
            (".com", false),
            ("emile@usherbrooke.ca", true),
        ]);

        for entry in tests {
            println!("testing {}-{}", entry.0, entry.1);
            assert_eq!(super::is_academic(entry.0), entry.1)
        }
    }

    #[test]
    fn get_school_names_full() {
        let tests = HashMap::from([
            ("lreilly@cs.strath.ac.uk", "University of Strathclyde"),
            ("lreilly@fadi.at", "BRG Fadingerstraße Linz, Austria"),
            ("abadojack@students.uonbi.ac.ke", "University of Nairobi"),
            ("harvard.edu", "Harvard University"),
            ("stanford.edu", "Stanford University"),
        ]);

        for (email, school_name) in tests {
            assert_eq!(school_name, super::get_school_names(email).unwrap()[0]);
        }
    }
}

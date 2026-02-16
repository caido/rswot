include!(concat!(env!("OUT_DIR"), "/tlds.rs"));
include!(concat!(env!("OUT_DIR"), "/stoplist.rs"));
include!(concat!(env!("OUT_DIR"), "/abused.rs"));
include!(concat!(env!("OUT_DIR"), "/institutions.rs"));

pub fn get_institution_names(parts: &[&str]) -> Option<Vec<String>> {
    let mut key = parts[0].to_string();

    for part in parts.iter().skip(1) {
        key.push_str("/");
        key.push_str(part);
        if let Some(names) = INSTITUTIONS.get(&key) {
            return Some(names.iter().map(|s| s.to_string()).collect());
        }
    }

    return None;
}

pub fn is_under_tld(parts: &[&str]) -> bool {
    check_set(&TLDS, &parts)
}

pub fn is_stop_list(parts: &[&str]) -> bool {
    check_set(&STOPLIST, &parts)
}

pub fn is_abused(parts: &[&str]) -> bool {
    check_set(&ABUSED, &parts)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_under_tld() {
        assert!(!is_under_tld(&["es", "ugr"]));
    }

    #[test]
    fn test_is_stop_list() {
        assert!(!is_stop_list(&["es", "ugr"]));
    }

    #[test]
    fn test_get_institution_names() {
        let tests = Vec::from([
            ("cs.strath.ac.uk", "University of Strathclyde"),
            ("fadi.at", "BRG Fadingerstraße Linz, Austria"),
            ("abadojack@students.uonbi.ac.ke", "University of Nairobi"),
            ("harvard.edu", "Harvard University"),
            ("stanford.edu", "Stanford University"),
        ]);

        for (domain, name) in tests {
            let parts = domain.split(".").collect::<Vec<&str>>();
            assert_eq!(get_institution_names(&parts).unwrap()[0], name);
        }
    }
}

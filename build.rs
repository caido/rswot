use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::{env, io};

use phf_codegen::{Map, Set};

const SKIP_FILES: &[&str] = &["tlds.txt", "abused.txt", "stoplist.txt"];

fn create_institutions(domains_dir: &Path) {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("institutions.rs");
    let mut file = BufWriter::new(File::create(&out_path).unwrap());

    let mut map = Map::<String>::new();

    fn walk(dir: &Path, domains_root: &Path, map: &mut phf_codegen::Map<String>) {
        if let Ok(rd) = fs::read_dir(dir) {
            for e in rd.flatten() {
                let path = e.path();
                if path.is_dir() {
                    walk(&path, domains_root, map);
                } else if path.extension().is_some_and(|e| e == "txt") {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    let parent = path.parent().unwrap_or(dir);
                    if parent == domains_root && SKIP_FILES.contains(&name) {
                        continue;
                    }
                    let key = path
                        .strip_prefix(domains_root)
                        .unwrap_or(&path)
                        .with_extension("");
                    let key = key.to_string_lossy().replace('\\', "/");

                    let lines: Vec<String> = io::BufReader::new(File::open(&path).unwrap())
                        .lines()
                        .map_while(Result::ok)
                        .map(|l| l.trim().to_string())
                        .filter(|l| !l.is_empty())
                        .collect();

                    let value_lit = format!(
                        "&[{}]",
                        lines
                            .iter()
                            .map(|l| format!("{:?}", l))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    map.entry(key, value_lit);
                }
            }
        }
    }

    walk(domains_dir, domains_dir, &mut map);

    writeln!(
        &mut file,
        "static INSTITUTIONS: phf::Map<&'static str, &'static [&'static str]> = \n{};\n",
        map.build()
    )
    .unwrap();
}

fn create_set(file_path: &Path, set_name: &str) {
    let lines = File::open(file_path).map(|file| io::BufReader::new(file).lines());
    if let Ok(lines) = lines {
        let set_source_path =
            Path::new(&env::var("OUT_DIR").unwrap()).join(format!("{}.rs", set_name));
        let mut file = BufWriter::new(File::create(&set_source_path).unwrap());
        let mut set: Set<String> = Set::new();
        let mut duplicates = HashSet::new();

        for line in lines {
            if let Ok(l) = line {
                if duplicates.insert(l.clone()) {
                    set.entry(l);
                }
            } else {
                println!("Error on {} set line {:?}", set_name, line);
            }
        }

        writeln!(
            &mut file,
            "static {}: phf::Set<&'static str> = \n{};\n",
            set_name.to_uppercase(),
            set.build()
        )
        .unwrap();
    } else {
        println!("Can not read {} file", set_name);
    }
}

fn main() {
    create_set(&Path::new("domains").join("tlds.txt"), "tlds");
    create_set(&Path::new("domains").join("abused.txt"), "abused");
    create_set(&Path::new("domains").join("stoplist.txt"), "stoplist");
    create_institutions(Path::new("domains"));
}

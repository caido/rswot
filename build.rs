use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::{env, io};

use phf_codegen::Set;

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
}

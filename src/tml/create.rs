use std::{fs::File, io::Write, path::Path};

pub fn create_toml(toml: &str, path: &Path) {
    let mut file = File::create(path).unwrap();
    file.write_all(toml.as_bytes()).unwrap();
}

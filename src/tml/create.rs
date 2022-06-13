use std::{fs::File, io::Write};

pub fn create_toml(toml: &str) {
    let mut file = File::create("./setting.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();
}

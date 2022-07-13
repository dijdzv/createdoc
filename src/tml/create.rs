use std::{fs::File, io::Write, path::Path};

pub fn create_toml(toml: &str, path: &Path) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(toml.as_bytes())?;

    Ok(())
}

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub fn create_toml<P: AsRef<Path>>(toml: &str, path: P) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(toml.as_bytes())?;

    Ok(())
}

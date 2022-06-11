use super::read_toml;
use std::fs;
use std::io;
use std::path::Path;

pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let setting = read_toml();
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}
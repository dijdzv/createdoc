use super::read_dir;
use std::io;
use std::path::Path;

pub fn read_control<P: AsRef<Path>>(
    path: P,
    ext: &str,
    ex_filename: &[String],
    folder: &[String],
) -> io::Result<Vec<String>> {
    let current = read_dir(path, ext, ex_filename)?;
    Ok(current)
}

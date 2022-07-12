use super::read_dir;
use std::io;
use std::path::Path;

pub fn read_control<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_filename: &[String],
    folder: &[String],
) -> io::Result<Vec<String>> {
    let mut current = read_dir(&path, ext, exclude_filename)?;
    // if !folder.contains(&"*".to_string()) {
    for f in folder {
        let mut file = read_dir(path.as_ref().join(f), ext, exclude_filename)?;
        current.append(&mut file);
    }
    // }

    Ok(current)
}

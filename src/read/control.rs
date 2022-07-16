use super::read_dir;
use std::io;
use std::path::Path;

pub fn read_control<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_filename: &[String],
    read_folder: &[String],
) -> io::Result<Vec<String>> {
    let (mut current, exist_folder) = read_dir(&path, ext, exclude_filename)?;
    if !read_folder.contains(&"*".to_string()) {
        for f in read_folder {
            let (mut file, _) = read_dir(path.as_ref().join(f), ext, exclude_filename)?;
            current.append(&mut file);
        }
    } else {
        for f in &exist_folder {
            let (mut file, _) = read_dir(path.as_ref().join(f), ext, exclude_filename)?;
            current.append(&mut file);
        }
    }

    Ok(current)
}

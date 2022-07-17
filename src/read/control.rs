use super::read_dir;
use std::io;
use std::path::Path;

fn read_recursive<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_filename: &[String],
    read_folder: &[String],
    current: &mut Vec<String>,
) -> io::Result<()> {
    let (mut read_filename, exist_folder) = read_dir(&path, ext, exclude_filename)?;
    current.append(&mut read_filename);
    dbg!(path.as_ref(), &exist_folder);
    if exist_folder.is_empty() {
        return Ok(());
    }
    if !read_folder.contains(&"*".to_string()) {
        for f in read_folder {
            read_recursive(
                path.as_ref().join(f),
                ext,
                exclude_filename,
                read_folder,
                current,
            )?;
        }
    } else {
        for f in &exist_folder {
            read_recursive(f, ext, exclude_filename, read_folder, current)?;
        }
    }

    Ok(())
}

pub fn read_control<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_filename: &[String],
    read_folder: &[String],
) -> io::Result<Vec<String>> {
    let mut current = Vec::new();
    read_recursive(&path, ext, exclude_filename, read_folder, &mut current)?;

    Ok(current)
}

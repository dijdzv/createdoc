use super::read_dir;
use std::io;
use std::path::Path;

fn read_recursive<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_file: &[String],
    read_folder: &[String],
    current: &mut Vec<String>,
) -> io::Result<()> {
    if !path.as_ref().exists() {
        return Ok(());
    }
    let (mut read_filename, exist_folder) = read_dir(&path, ext, exclude_file)?;
    current.append(&mut read_filename);
    if exist_folder.is_empty() {
        return Ok(());
    }
    if read_folder.contains(&"*".to_string()) {
        for f in &exist_folder {
            read_recursive(f, ext, exclude_file, read_folder, current)?;
        }
    } else {
        for f in read_folder {
            read_recursive(
                path.as_ref().join(f),
                ext,
                exclude_file,
                read_folder,
                current,
            )?;
        }
    }

    Ok(())
}

pub fn read_control<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_file: &[String],
    read_folder: &[String],
) -> io::Result<Vec<String>> {
    let mut current = Vec::new();
    read_recursive(&path, ext, exclude_file, read_folder, &mut current)?;

    Ok(current)
}

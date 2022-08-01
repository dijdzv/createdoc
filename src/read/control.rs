use super::read_dir;
use crate::Setting;

use std::{io, path::Path};

fn read_recursive<P: AsRef<Path>>(
    current: &mut Vec<String>,
    path: P,
    ext: &str,
    read_folder: &[String],
    (exclude_file, exclude_folder): (&[String], &[String]),
) -> io::Result<()> {
    if !path.as_ref().exists() {
        return Ok(());
    }

    let (mut read_filename, exist_folder) = read_dir(&path, ext, (exclude_file, exclude_folder))?;
    current.append(&mut read_filename);
    if exist_folder.is_empty() {
        return Ok(());
    }
    if read_folder.contains(&"*".to_string()) {
        for f in &exist_folder {
            read_recursive(current, f, ext, read_folder, (exclude_file, exclude_folder))?;
        }
    } else {
        for f in read_folder {
            if f.is_empty() {
                continue;
            };
            read_recursive(
                current,
                path.as_ref().join(f),
                ext,
                read_folder,
                (exclude_file, exclude_folder),
            )?;
        }
    }

    Ok(())
}

pub fn read_control(setting: &Setting) -> io::Result<Vec<String>> {
    let mut current = Vec::new();
    read_recursive(
        &mut current,
        setting.read_dir(),
        setting.read_ext(),
        setting.read_folder(),
        setting.exclude_tuple(),
    )?;

    Ok(current)
}

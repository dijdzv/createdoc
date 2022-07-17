use std::ffi::OsStr;
use std::path::Path;
use std::{fs, io};

pub fn read_dir<P: AsRef<Path>>(
    path: P,
    ext: &str,
    (exclude_file, exclude_folder): (&[String], &[String]),
) -> io::Result<(Vec<String>, Vec<String>)> {
    let mut folder = Vec::new();
    Ok((
        fs::read_dir(&path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let f_name = entry.file_name().to_string_lossy().into_owned();
                let stem = Path::new(&f_name).file_stem()?.to_str()?;
                let filepath = path.as_ref().join(&f_name).to_string_lossy().into_owned();
                if entry.file_type().ok()?.is_file()
                    && Path::new(&f_name).extension() == Some(OsStr::new(ext))
                    && !exclude_file.iter().any(|f| stem == f)
                {
                    Some(filepath)
                } else if entry.file_type().ok()?.is_dir()
                    && !exclude_folder.iter().any(|f| f_name == *f)
                {
                    folder.push(filepath);
                    None
                } else {
                    None
                }
            })
            .collect(),
        folder,
    ))
}

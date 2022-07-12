use std::fs;
use std::io;
use std::path::Path;

pub fn read_dir<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_filename: &[String],
) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(&path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let filename = entry.file_name();
            let filename = filename.to_string_lossy().into_owned();
            if entry.file_type().ok()?.is_file()
                && Path::new(&filename).extension().unwrap().to_str().unwrap() == ext
                && exclude_filename.iter().any(|f| !filename.starts_with(f))
            {
                Some(path.as_ref().join(filename).to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

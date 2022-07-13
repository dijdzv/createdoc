use std::fs;
use std::io;
use std::path::Path;

pub fn read_dir<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_filename: &[String],
) -> io::Result<(Vec<String>, Vec<String>)> {
    let mut folder = Vec::new();
    Ok((
        fs::read_dir(&path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let filename = entry.file_name();
                let filename = filename.to_string_lossy().into_owned();
                let filepath = path.as_ref().join(&filename).to_string_lossy().into_owned();
                if entry.file_type().ok()?.is_file()
                    && Path::new(&filename).extension() == Path::new(ext).extension()
                    && exclude_filename.iter().any(|f| !filename.starts_with(f))
                {
                    Some(filepath)
                } else if entry.file_type().ok()?.is_dir() {
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

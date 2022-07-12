use std::fs;
use std::io;
use std::path::Path;

pub fn read_dir<P: AsRef<Path>>(
    path: P,
    ext: &str,
    exclude_filename: &[String],
) -> io::Result<Vec<String>> {
    // let path = path.as_ref();
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if let Ok(file_type) = entry.file_type() {
                let filename = entry.file_name();
                let f_path = Path::new(&filename);
                let filename = filename.to_string_lossy().into_owned();
                if file_type.is_file()
                    && f_path.extension().unwrap().to_str().unwrap() == ext
                    && exclude_filename.iter().any(|f| !filename.starts_with(f))
                {
                    Some(filename)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect())
}

use std::path::Path;

use crate::error::ErrorMsg;
use createdoc::ReadData;

pub fn add_file<P: AsRef<Path>>(
    read_data: &mut ReadData,
    filepath: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = filepath
        .as_ref()
        .file_name()
        .ok_or_else(|| ErrorMsg::FileName.as_str())?
        .to_string_lossy()
        .into_owned();
    let parent_filepath = filepath
        .as_ref()
        .parent()
        .ok_or_else(|| ErrorMsg::Parent.as_str())?;
    if parent_filepath == Path::new(&read_data.read_dir) {
        read_data.push_dir_vec(filename);
    } else {
        let parent_name = parent_filepath
            .strip_prefix(Path::new(&read_data.read_dir))?
            .to_str()
            .ok_or_else(|| ErrorMsg::ToStr.as_str())?
            .replace('\\', "::");
        read_data.push_dir_vec(format!("{}::{}", parent_name, filename));
    }
    read_data.clear_file_vec();

    Ok(())
}

use anyhow::Context;
use std::path::Path;

use crate::error::ErrorMsg;
use createdoc::ReadData;

pub fn add_file<P: AsRef<Path>>(read_data: &mut ReadData, filepath: P) -> anyhow::Result<()> {
    if read_data.is_empty_file_vec() {
        return Ok(());
    }
    let filename = filepath
        .as_ref()
        .file_name()
        .with_context(|| ErrorMsg::FileName.as_str())?
        .to_string_lossy()
        .into_owned();
    let parent_filepath = filepath
        .as_ref()
        .parent()
        .with_context(|| ErrorMsg::Parent.as_str())?;
    if parent_filepath == Path::new(&read_data.read_dir) {
        read_data.push_all(filename);
    } else {
        let parent_name = parent_filepath
            .strip_prefix(Path::new(&read_data.read_dir))?
            .to_str()
            .with_context(|| ErrorMsg::ToStr.as_str())?
            .replace('\\', "::")
            .replace('/', "::");
        let dbg = parent_filepath
            .strip_prefix(Path::new(&read_data.read_dir))?
            .components()
            .next()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();
        dbg!(dbg);
        read_data.push_all(format!("{}", dbg));
        // read_data.push_all(format!("{}::{}", parent_name, filename));
    }
    read_data.clear_file_vec();

    Ok(())
}

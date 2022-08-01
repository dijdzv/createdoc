use anyhow::Context;
use std::path::Path;

use crate::error::ErrorMsg;
use crate::ReadData;

pub fn add_file<P: AsRef<Path>>(read_data: &mut ReadData, filepath: P) -> anyhow::Result<()> {
    if read_data.is_empty_file_vec() {
        return Ok(());
    }
    let filename = filepath
        .as_ref()
        .file_stem()
        .with_context(|| ErrorMsg::FileStem.as_str())?
        .to_string_lossy()
        .into_owned();
    let parent_filepath = filepath
        .as_ref()
        .parent()
        .with_context(|| ErrorMsg::Parent.as_str())?;
    if parent_filepath == Path::new(&read_data.read_dir) {
        read_data.push_all(filename);
    } else {
        let name = if read_data.is_module {
            parent_filepath
                .strip_prefix(Path::new(&read_data.read_dir))?
                .components()
                .next()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .into_owned()
        } else {
            format!(
                "{}::{}",
                parent_filepath
                    .strip_prefix(Path::new(&read_data.read_dir))?
                    .to_string_lossy()
                    .into_owned()
                    .replace('\\', "::")
                    .replace('/', "::"),
                filename
            )
        };
        read_data.push_all(name);
    }
    read_data.clear_file_vec();

    Ok(())
}

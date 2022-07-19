use std::fs::File;
use std::io::{BufRead, BufReader};

use super::{add_file, add_line};
use createdoc::ReadData;

pub fn add_control(filepaths: Vec<String>, read_data: &mut ReadData) -> anyhow::Result<()> {
    // folderに格納
    for filepath in &filepaths {
        // fileに格納
        for result in BufReader::new(File::open(filepath)?).lines() {
            // 一行のデータ
            read_data.line = result?;

            // vecに行毎追加
            add_line(read_data)?;
        }

        add_file(read_data, filepath)?;
    }

    read_data.sort_dir_vec();

    Ok(())
}

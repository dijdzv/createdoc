mod add;
mod create;
mod error;
mod read;
mod tml;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use createdoc::ReadData;
use error::ErrorMsg;

fn main() {
    match app() {
        Ok(create_filepath) => println!("Created `{}` successfully!", create_filepath),
        Err(e) => {
            println!("{}", e)
        }
    }
}

fn app() -> Result<String, Box<dyn std::error::Error>> {
    let setting = tml::read_toml()?;
    let read_dir = setting.read_dir();
    let filepaths = read::read_control(
        read_dir,
        setting.read_ext(),
        setting.exclude_filename(),
        setting.read_folder(),
    )?;

    let mut read_data = ReadData::new(&setting);

    // folderに格納
    for filepath in &filepaths {
        // fileに格納
        let filepath = Path::new(&filepath);
        for result in BufReader::new(File::open(filepath)?).lines() {
            // 一行のデータ
            read_data.line = result?;

            // vecに行毎追加
            add::add_line(&mut read_data)?;
        }
        let filename = filepath
            .file_name()
            .ok_or_else(|| ErrorMsg::FileName.as_str())?
            .to_string_lossy()
            .into_owned();
        if filepath.parent().ok_or_else(|| ErrorMsg::Parent.as_str())? == Path::new(read_dir) {
            read_data.push_dir_vec(filename);
        } else {
            let parent_name = filepath
                .parent()
                .ok_or_else(|| ErrorMsg::Parent.as_str())?
                .file_name()
                .ok_or_else(|| ErrorMsg::FileName.as_str())?
                .to_str()
                .ok_or_else(|| ErrorMsg::ToStr.as_str())?;
            read_data.push_dir_vec(format!("{}::{}", parent_name, filename));
        }
        read_data.clear_file_vec();
    }

    read_data.sort_dir_vec();

    let read_lang = setting.read_lang();
    create::create_html(setting.create_dir(), read_lang, &read_data.dir_vec)?;

    Ok(format!("{}doc.html", read_lang))
}

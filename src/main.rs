mod add;
mod create;
mod error;
mod read;
mod sort;
mod tml;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use error::ErrorMsg;

type TargetName = String;
type FileName = String;
type Doc = Vec<String>;
type Content = Vec<String>;
type FileVec = Vec<(TargetName, Doc, Content)>;
type FolderVec = Vec<(FileName, FileVec)>;

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

    let mut doc = Vec::new(); // 一時保管
    let mut content = Vec::new(); //docとfuncのペア
    let mut file_vec: FileVec = Vec::new(); // １つのfile
    let mut folder_vec: FolderVec = Vec::new(); // 全てのファイル
    let mut target_name = String::from(""); // 関数名のbuf
    let mut is_doc = false;
    let mut is_content = false;

    let cmt_start = setting.cmt_start();
    let target_list = setting.target_list();
    let read_dir = setting.read_dir();
    let filepaths = read::read_control(
        read_dir,
        setting.read_ext(),
        setting.exclude_filename(),
        setting.read_folder(),
    )?;

    // folderに格納
    for filepath in &filepaths {
        // fileに格納
        let filepath = Path::new(&filepath);
        for result in BufReader::new(File::open(filepath)?).lines() {
            // 一行
            let mut l = result?;

            // vecに行毎追加
            add::add_line(
                &mut l,
                (&mut is_doc, &mut is_content),
                (&mut doc, &mut content, &mut target_name, &mut file_vec),
                cmt_start,
                target_list,
            )?;
        }
        let filename = filepath
            .file_name()
            .ok_or_else(|| ErrorMsg::FileName.as_str())?
            .to_string_lossy()
            .into_owned();
        if filepath.parent().ok_or_else(|| ErrorMsg::Parent.as_str())? == Path::new(read_dir) {
            folder_vec.push((filename, file_vec.clone()));
        } else {
            let parent_name = filepath
                .parent()
                .ok_or_else(|| ErrorMsg::Parent.as_str())?
                .file_name()
                .ok_or_else(|| ErrorMsg::FileName.as_str())?
                .to_str()
                .ok_or_else(|| ErrorMsg::ToStr.as_str())?;
            folder_vec.push((format!("{}::{}", parent_name, filename), file_vec.clone()));
        }
        file_vec.clear();
    }

    folder_vec = sort::sort(&mut folder_vec);

    let read_lang = setting.read_lang();
    create::create_html(setting.create_dir(), read_lang, &folder_vec)?;

    Ok(format!("{}doc.html", read_lang))
}

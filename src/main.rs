mod add;
mod create;
mod read;
mod sort;
mod tml;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type TargetName = String;
type FileName = String;
type Doc = Vec<String>;
type Content = Vec<String>;
type FileVec = Vec<(TargetName, Doc, Content)>;
type FolderVec = Vec<(FileName, FileVec)>;

fn main() {
    match app() {
        Ok(_) => println!("Creation Success!"),
        Err(e) => {
            tml::create_toml(tml::TOML, Path::new(tml::TOML_PATH));
            println!("The `setting.toml` file did not exist, so I created a new one.");
            println!("{}", e)
        }
    }
}

fn app() -> Result<(), Box<dyn std::error::Error>> {
    let (
        (read_dir, read_folder, create_dir),
        cmt_start,
        (read_lang, read_ext, exclude_filename),
        target,
    ) = tml::read_toml()?;

    let mut doc = Vec::new(); // 一時保管
    let mut content = Vec::new(); //docとfuncのペア
    let mut file_vec: FileVec = Vec::new(); // １つのfile
    let mut folder_vec: FolderVec = Vec::new(); // 全てのファイル
    let mut target_name = String::from(""); // 関数名のbuf
    let mut is_doc = false;
    let mut is_content = false;

    let filepaths = read::read_control(&read_dir, &read_ext, &exclude_filename, &read_folder)?;

    // folderに格納
    for filepath in &filepaths {
        // fileに格納
        let filepath = Path::new(&filepath);
        for result in BufReader::new(File::open(filepath).unwrap()).lines() {
            // 一行
            let mut l = result.ok().unwrap();

            // vecに行毎追加
            add::add_line(
                &mut l,
                (&mut is_doc, &mut is_content),
                (&mut doc, &mut content, &mut target_name, &mut file_vec),
                &cmt_start,
                &target,
            );
        }
        let filename = filepath.file_name().unwrap().to_string_lossy().into_owned();
        if filepath.parent().unwrap() == Path::new(&read_dir) {
            folder_vec.push((filename, file_vec.clone()));
        } else {
            let parent_dir = filepath
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            folder_vec.push((format!("{}::{}", parent_dir, filename), file_vec.clone()));
        }
        file_vec.clear();
    }

    folder_vec = sort::sort(&mut folder_vec);

    create::create_html(&create_dir, &read_lang, &folder_vec);

    Ok(())
}

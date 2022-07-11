mod add;
mod create;
mod exclude;
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
        (read_dir, create_dir),
        cmt_start,
        (read_lang, read_filename_extension, ex_filename),
        target,
    ) = tml::read_toml()?;

    let mut doc = Vec::new(); // 一時保管
    let mut content = Vec::new(); //docとfuncのペア
    let mut file_vec: FileVec = Vec::new(); // １つのfile
    let mut folder_vec: FolderVec = Vec::new(); // 全てのファイル
    let mut target_name = String::from(""); // 関数名のbuf
    let mut is_doc = false;
    let mut is_content = false;

    let mut filenames = read::read_dir(read_dir).unwrap();
    filenames = exclude::exclude(filenames, &read_filename_extension, ex_filename);

    // folderに格納
    for filename in &filenames {
        // fileに格納
        for result in BufReader::new(File::open(filename).unwrap()).lines() {
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
        folder_vec.push((filename.to_string(), file_vec.clone()));
        file_vec.clear();
    }

    create::create_html(&create_dir, &read_lang, &folder_vec);

    Ok(())
}

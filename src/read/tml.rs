use serde_derive::*;
use std::fs::read_to_string;
use toml::{self, de};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    dir: Dir,
    file: File,
    exclude_file: ExcludeFile,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dir {
    read_dir: Option<String>,
    create_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    filename: Option<Vec<String>>,
    filename_extension: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExcludeFile {
    ex_filename: Option<Vec<String>>,
    ex_filename_extension: Option<Vec<String>>,
}

pub fn read_toml() -> Setting {
    let s = match read_to_string("./setting.toml") {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    let setting: Result<Setting, de::Error> = toml::from_str(&s);
    match &setting {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("fail to parse toml: {}", e),
    };

    setting.unwrap()

    // let Setting {
    //     dir,
    //     file,
    //     exclude_file,
    // } = setting.unwrap();

    // let (
    //     Dir {
    //         read_dir,
    //         create_dir,
    //     },
    //     File {
    //         filename,
    //         filename_extension,
    //     },
    //     ExcludeFile {
    //         ex_filename,
    //         ex_filename_extension,
    //     },
    // ) = (dir, file, exclude_file);
}

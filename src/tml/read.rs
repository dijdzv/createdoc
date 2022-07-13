use super::TOML_PATH;
use serde_derive::*;
use std::fs::read_to_string;
use toml::{self, de};
#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    dir: Dir,
    read: Read,
    exclude: Exclude,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dir {
    read_dir: String,
    read_folder: Vec<String>,
    create_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Read {
    read_lang: String,
    read_ext: String,
    cmt_start: String,
    target: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exclude {
    exclude_filename: Vec<String>,
}

type ReadType = (String, String, Vec<String>);
type Ok = ((String, Vec<String>, String), String, ReadType, Vec<String>);

pub fn read_toml() -> Result<Ok, Box<dyn std::error::Error>> {
    let s = read_to_string(TOML_PATH)?;

    let setting: Result<Setting, de::Error> = toml::from_str(&s);
    // match &setting {
    //     Ok(p) => println!("{:#?}", p),
    //     Err(e) => panic!("fail to parse toml: {}", e),
    // };

    let Setting { dir, read, exclude } = setting?;

    let (
        Dir {
            read_dir,
            read_folder,
            create_dir,
        },
        Read {
            read_lang,
            read_ext,
            cmt_start,
            target,
        },
        Exclude { exclude_filename },
    ) = (dir, read, exclude);

    Ok((
        (read_dir, read_folder, create_dir),
        cmt_start,
        (read_lang, read_ext, exclude_filename),
        target,
    ))
}

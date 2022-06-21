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
    create_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Read {
    read_lang: String,
    read_filename_extension: String,
    cmt_start: String,
    target: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exclude {
    ex_filename: Vec<String>,
}

type ReadType = (String, String, Vec<String>);
type Ok = ((String, String), String, ReadType, Vec<String>);
type Error = Box<dyn std::error::Error>;

pub fn read_toml() -> Result<Ok, Error> {
    let s = read_to_string(TOML_PATH)?;

    let setting: Result<Setting, de::Error> = toml::from_str(&s);
    // match &setting {
    //     Ok(p) => println!("{:#?}", p),
    //     Err(e) => panic!("fail to parse toml: {}", e),
    // };

    let Setting { dir, read, exclude } = setting.unwrap();

    let (
        Dir {
            read_dir,
            create_dir,
        },
        Read {
            read_lang,
            read_filename_extension,
            cmt_start,
            target,
        },
        Exclude { ex_filename },
    ) = (dir, read, exclude);

    Ok((
        (read_dir, create_dir),
        cmt_start,
        (read_lang, read_filename_extension, ex_filename),
        target,
    ))
}

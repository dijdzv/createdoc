use super::{create_toml, TOML};
use serde_derive::*;
use std::fs::read_to_string;
use std::path::Path;
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
    cmt_end: String,
    target: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exclude {
    ex_filename: Vec<String>,
}

type TupleString = (String, String);

pub fn read_toml() -> (
    TupleString,
    TupleString,
    (String, String, Vec<String>),
    Vec<String>,
) {
    let path = Path::new("./setting.toml");
    let s = match read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            create_toml(TOML, path);
            panic!("The `setting.toml` file did not exist, so I created a new one.");
        }
    };

    let setting: Result<Setting, de::Error> = toml::from_str(&s);
    match &setting {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("fail to parse toml: {}", e),
    };

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
            cmt_end,
            target,
        },
        Exclude { ex_filename },
    ) = (dir, read, exclude);

    (
        (read_dir, create_dir),
        (cmt_start, cmt_end),
        (read_lang, read_filename_extension, ex_filename),
        target,
    )
}

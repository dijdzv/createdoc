use serde_derive::*;
use std::fs::read_to_string;
use std::path::Path;
use toml::{self, de};
#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    file: File,
    dir: Dir,
    read: Read,
    exclude: Exclude,
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    create_filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dir {
    read_dir: String,
    create_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Read {
    read_filename_extension: String,
    doc_start: String,
    doc_end: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exclude {
    ex_filename: Vec<String>,
}

pub fn read_toml() -> (String, String, String, String, String, String, Vec<String>) {
    let path = Path::new("./setting.toml");
    let s = match read_to_string(path) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    let setting: Result<Setting, de::Error> = toml::from_str(&s);
    match &setting {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("fail to parse toml: {}", e),
    };

    let Setting {
        file,
        dir,
        read,
        exclude,
    } = setting.unwrap();

    let (
        File { create_filename },
        Dir {
            read_dir,
            create_dir,
        },
        Read {
            read_filename_extension,
            doc_start,
            doc_end,
        },
        Exclude { ex_filename },
    ) = (file, dir, read, exclude);

    (
        create_filename,
        read_dir,
        create_dir,
        read_filename_extension,
        doc_start,
        doc_end,
        ex_filename,
    )
}

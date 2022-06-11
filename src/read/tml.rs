use serde_derive::*;
use std::fs::read_to_string;
use std::path::Path;
use toml::{self, de};

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    dir: Dir,
    file: File,
    exclude_file: ExcludeFile,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dir {
    read_dir: String,
    create_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    read_filename: Vec<String>,
    read_filename_extension: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExcludeFile {
    ex_filename: Vec<String>,
}

pub fn read_toml() -> (String, String, Vec<String>, String, Vec<String>) {
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
        dir,
        file,
        exclude_file,
    } = setting.unwrap();

    let (
        Dir {
            read_dir,
            create_dir,
        },
        File {
            read_filename,
            read_filename_extension,
        },
        ExcludeFile { ex_filename },
    ) = (dir, file, exclude_file);

    (
        read_dir,
        create_dir,
        read_filename,
        read_filename_extension,
        ex_filename,
    )
}

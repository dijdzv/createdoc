use crate::tml;
use serde_derive::*;
use std::{
    fs::read_to_string,
    io::{self, ErrorKind},
    path::Path,
};

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

pub fn read_toml() -> Result<Ok, io::Error> {
    let result = read_to_string(tml::TOML_PATH);
    let s = match result {
        Ok(s) => s,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            tml::create_toml(tml::TOML, Path::new(tml::TOML_PATH))?;
            let new_e = io::Error::new(
                ErrorKind::NotFound,
                "The `setting.toml` file did not exist, so I created a new one.",
            );
            return Err(new_e);
        }
        Err(e) => return Err(e),
    };

    let Setting { dir, read, exclude } = toml::from_str(&s)?;

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

use serde_derive::*;
use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
};

/// ファイル出力用のstruct
pub struct Output {
    code: String,
}

impl Output {
    pub fn add<T: Display>(&mut self, code: T) {
        *self = Self {
            code: format!("{}{}", self.code, code),
        };
    }
    pub fn new() -> Self {
        Output {
            code: String::new(),
        }
    }
    pub fn write(&self, file: &mut File) -> io::Result<()> {
        file.write_all(self.code.as_bytes())
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

/// toml用のstruct
#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
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
    target_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exclude {
    exclude_filename: Vec<String>,
}

impl Setting {
    pub fn cmt_start(&self) -> &str {
        &self.read.cmt_start
    }
    pub fn create_dir(&self) -> &str {
        &self.dir.create_dir
    }
    pub fn exclude_filename(&self) -> &[String] {
        &self.exclude.exclude_filename
    }
    pub fn read_dir(&self) -> &str {
        &self.dir.read_dir
    }
    pub fn read_ext(&self) -> &str {
        &self.read.read_ext
    }
    pub fn read_folder(&self) -> &[String] {
        &self.dir.read_folder
    }
    pub fn read_lang(&self) -> &str {
        &self.read.read_lang
    }
    pub fn target_list(&self) -> &[String] {
        &self.read.target_list
    }
}

// pub struct Data {
//     line: String,
//     doc: Doc,
//     content: Content,
//     target_name: TargetName,
//     file_vec: FileVec,
//     all_vec: Vec<(String, FileVec)>,
//     cmt_start: String,
//     target_list: Vec<String>,
//     is_doc: bool,
//     is_content: bool,
// }
// struct FileVec {
//     file_vec: Vec<(TargetName, Doc, Content)>,
// }
// struct TargetName {
//     target_name: String,
// }
// struct Doc {
//     doc: Vec<String>,
// }
// struct Content {
//     content: Vec<String>,
// }

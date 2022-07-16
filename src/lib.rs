mod sort;

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
}

/// file_vec: Vec<(TargetName, Doc, Content)>
/// dir_vec: Vec<(FileName, FileVec)>
#[derive(Debug)]
pub struct ReadData {
    pub line: String,
    pub doc: Vec<String>,
    pub content: Vec<String>,
    pub target_name: String,
    pub file_vec: Vec<(String, Vec<String>, Vec<String>)>,
    pub dir_vec: DirVec,
    pub cmt_start: String,
    pub target_list: Vec<String>,
    pub is_doc: bool,
    pub is_content: bool,
}
pub type DirVec = Vec<(String, Vec<(String, Vec<String>, Vec<String>)>)>;

impl ReadData {
    pub fn clear_content(&mut self) {
        self.content.clear();
    }
    pub fn clear_doc(&mut self) {
        self.doc.clear();
    }
    pub fn clear_file_vec(&mut self) {
        self.file_vec.clear();
    }
    pub fn new(setting: &Setting) -> Self {
        Self {
            line: String::new(),
            doc: Vec::new(),
            content: Vec::new(),
            target_name: String::new(),
            file_vec: Vec::new(),
            dir_vec: Vec::new(),
            cmt_start: setting.read.cmt_start.to_string(),
            target_list: setting.read.target_list.to_vec(),
            is_doc: false,
            is_content: false,
        }
    }
    pub fn push_content(&mut self, line: &str) {
        self.content.push(line.to_string());
    }
    pub fn push_dir_vec(&mut self, filename: String) {
        self.dir_vec.push((filename, self.file_vec.to_owned()));
    }
    pub fn push_doc(&mut self, line: &str) {
        self.doc.push(line.to_string());
    }
    pub fn push_file_vec(&mut self) {
        self.file_vec.push((
            self.target_name.to_owned(),
            self.doc.to_owned(),
            self.content.to_owned(),
        ))
    }
    pub fn sort_dir_vec(&mut self) {
        self.dir_vec = sort::sort(&mut self.dir_vec);
    }
    pub fn start_content(&mut self, line: &str, target: &str) {
        if self.is_doc {
            self.is_doc = false;
        }
        self.push_content(line);
        self.is_content = true; // content start
        self.line = self.line.replacen(target, "", 1);
    }
}
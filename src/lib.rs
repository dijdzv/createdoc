use serde_derive::*;
use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
    path::Path,
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
    name: Name,
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
    modifier: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exclude {
    exclude_file: Vec<String>,
    exclude_folder: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    create_filename: String,
}

impl Setting {
    pub fn combine_modifier_and_target_list(&self) -> Vec<String> {
        let mut v = Vec::new();
        for t in &self.read.target_list {
            v.push(t.to_owned());
            for m in &self.read.modifier {
                v.push(format!("{} {}", m, t));
            }
        }
        v
    }
    pub fn create_dir(&self) -> &str {
        &self.dir.create_dir
    }
    pub fn create_filename(&self) -> &str {
        &self.name.create_filename
    }
    pub fn create_filepath(&self) -> String {
        if self.is_exist_create_filename() {
            format!(
                "{}.html",
                Path::new(self.create_dir())
                    .join(self.create_filename())
                    .display()
            )
        } else {
            format!(
                "{}doc.html",
                Path::new(self.create_dir())
                    .join(self.read_lang())
                    .display()
            )
        }
    }
    fn exclude_file(&self) -> &[String] {
        &self.exclude.exclude_file
    }
    fn exclude_folder(&self) -> &[String] {
        &self.exclude.exclude_folder
    }
    pub fn exclude_tuple(&self) -> (&[String], &[String]) {
        (self.exclude_file(), self.exclude_folder())
    }
    pub fn is_exist_create_filename(&self) -> bool {
        !self.name.create_filename.is_empty()
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
    doc: Vec<String>,
    content: Vec<String>,
    pub target_name: String,
    file_vec: Vec<(String, Vec<String>, Vec<String>)>,
    pub dir_vec: DirVec,
    pub cmt_start: String,
    pub target_list: Vec<String>,
    pub is_doc: bool,
    pub is_content: bool,
    pub read_dir: String,
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
    pub fn html_escape(&mut self) {
        self.line = self.line.replace('<', "&lt");
        self.line = self.line.replace('>', "&gt");
    }
    pub fn is_empty_file_vec(&self) -> bool {
        self.file_vec.is_empty()
    }
    pub fn new(setting: &Setting) -> Self {
        Self {
            line: String::new(),
            doc: Vec::new(),
            content: Vec::new(),
            target_name: String::new(),
            file_vec: Vec::new(),
            dir_vec: Vec::new(),
            cmt_start: setting.read.cmt_start.to_owned(),
            target_list: setting.combine_modifier_and_target_list(),
            is_doc: false,
            is_content: false,
            read_dir: setting.dir.read_dir.to_owned(),
        }
    }
    pub fn push_content(&mut self) {
        self.content.push(self.line.to_owned());
    }
    pub fn push_dir_vec(&mut self, filename: String) {
        self.dir_vec.push((filename, self.file_vec.to_owned()));
    }
    pub fn push_doc(&mut self) {
        self.doc.push(self.line.to_owned());
    }
    pub fn push_file_vec(&mut self) {
        self.file_vec.push((
            self.target_name.to_owned(),
            self.doc.to_owned(),
            self.content.to_owned(),
        ))
    }
    pub fn sort_dir_vec(&mut self) {
        let mut sorted_file = self
            .dir_vec
            .iter_mut()
            .map(|(f, file_vec)| {
                file_vec.sort_by(|a, b| a.0.cmp(&b.0));
                (f.to_owned(), file_vec.clone())
            })
            .collect::<Vec<_>>();

        sorted_file.sort_by(|a, b| a.0.cmp(&b.0));

        self.dir_vec = sorted_file;
    }
}

use serde_derive::*;
use std::{
    collections::HashMap,
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
    pub fn combine_modifier_and_target_list(&self) -> HashMap<String, Vec<String>> {
        let mut h = HashMap::new();
        for t in &self.read.target_list {
            let ptr = h.entry(t.to_owned()).or_insert_with(|| vec![t.to_owned()]);
            for m in &self.read.modifier {
                ptr.push(format!("{} {}", m, t));
            }
        }
        h
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

#[derive(Debug)]
pub struct ReadData {
    pub line: String,
    doc: Vec<String>,
    content: Vec<String>,
    pub target_name: TargetName,
    pub syntax: Syntax,
    file_vec: Vec<(Syntax, TargetName, Doc, Content)>,
    pub all: Vec<(Filename, FileVec)>,
    pub cmt_start: String,
    pub target_list: HashMap<Syntax, Vec<String>>,
    pub is_doc: bool,
    pub is_content: bool,
    pub read_dir: String,
}
type Syntax = String;
type TargetName = String;
type Doc = Vec<String>;
type Content = Vec<String>;
type Filename = String;
type FileVec = Vec<(Syntax, TargetName, Doc, Content)>;
pub type AllVec = Vec<(Filename, FileVec)>;
type Categorized<'a> = HashMap<&'a String, Vec<FileHash<'a>>>;
type FileHash<'a> = HashMap<&'a Filename, (&'a TargetName, &'a Doc, &'a Content)>;

impl ReadData {
    pub fn categorize_syntax(&self) -> Categorized {
        let mut syntax_hash: Categorized = HashMap::new();
        for (filename, file_vec) in &self.all {
            let mut file_hash: FileHash = HashMap::new();
            for (syntax, target_name, doc, content) in file_vec {
                file_hash.insert(filename, (target_name, doc, content));
                syntax_hash
                    .entry(syntax)
                    .and_modify(|e| e.push(file_hash.to_owned()))
                    .or_insert_with(|| vec![file_hash.to_owned()]);
            }
        }
        syntax_hash
    }
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
            syntax: String::new(),
            file_vec: Vec::new(),
            all: Vec::new(),
            cmt_start: setting.read.cmt_start.to_owned(),
            target_list: setting.combine_modifier_and_target_list(),
            is_doc: false,
            is_content: false,
            read_dir: setting.dir.read_dir.to_owned(),
        }
    }
    pub fn push_all(&mut self, filename: String) {
        self.all.push((filename, self.file_vec.to_owned()));
    }
    pub fn push_content(&mut self) {
        self.content.push(self.line.to_owned());
    }
    pub fn push_doc(&mut self) {
        self.doc.push(self.line.to_owned());
    }
    pub fn push_file_vec(&mut self) {
        self.file_vec.push((
            self.syntax.to_owned(),
            self.target_name.to_owned(),
            self.doc.to_owned(),
            self.content.to_owned(),
        ))
    }
    pub fn sort_all(&mut self) {
        let mut sorted_file = self
            .all
            .iter_mut()
            .map(|(f, file_vec)| {
                file_vec.sort_by(|a, b| a.1.cmp(&b.1));
                (f.to_owned(), file_vec.clone())
            })
            .collect::<Vec<_>>();

        sorted_file.sort_by(|a, b| a.0.cmp(&b.0));

        self.all = sorted_file;
    }
}

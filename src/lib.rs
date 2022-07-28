use serde_derive::*;
use std::{collections::BTreeMap, path::Path};

pub use output::*;

pub mod output {
    use std::{
        fmt::Display as D,
        fs::File,
        io::{self, Write},
    };
    /// ファイル出力用のstruct
    pub struct Output {
        code: String,
    }

    impl Output {
        pub fn add<T: D>(&mut self, code: T) {
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
}

/// toml用のstruct
#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    dir: Dir,
    read: Read,
    exclude: Exclude,
    display: Display,
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
struct Display {
    create_filename: String,
    is_module: bool,
}

impl Setting {
    pub fn combine_modifier_and_target_list(&self) -> BTreeMap<String, Vec<String>> {
        let mut map = BTreeMap::new();
        for t in &self.read.target_list {
            let ptr = map
                .entry(t.to_owned())
                .or_insert_with(|| vec![t.to_owned()]);
            for m in &self.read.modifier {
                ptr.push(format!("{} {}", m, t));
            }
        }
        map
    }
    pub fn create_dir(&self) -> &str {
        &self.dir.create_dir
    }
    pub fn create_filename(&self) -> &str {
        &self.display.create_filename
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
        !self.display.create_filename.is_empty()
    }
    pub fn is_module(&self) -> bool {
        self.display.is_module
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
    pub target_list: BTreeMap<Syntax, Vec<String>>,
    pub is_doc: bool,
    pub is_content: bool,
    pub read_dir: String,
    pub is_module: bool,
}
type Syntax = String;
type TargetName = String;
type Doc = Vec<String>;
type Content = Vec<String>;
type Filename = String;
type FileVec = Vec<(Syntax, TargetName, Doc, Content)>;
// &strはSyntax
type SyntaxMap<'a> = BTreeMap<&'a str, BTreeMap<&'a TargetName, (&'a Doc, &'a Content)>>;
pub type FileMap<'a> = BTreeMap<&'a Filename, SyntaxMap<'a>>;

impl ReadData {
    pub fn syntax_categorize(&self) -> FileMap {
        let mut mod_map: FileMap = BTreeMap::new();
        for (filename, file_vec) in &self.all {
            for (syntax, target_name, doc, content) in file_vec {
                mod_map
                    .entry(filename)
                    .and_modify(|e| {
                        e.entry(syntax)
                            .and_modify(|e| {
                                e.insert(target_name, (doc, content));
                            })
                            .or_insert_with(|| BTreeMap::from([(target_name, (doc, content))]));
                    })
                    .or_insert_with(|| {
                        BTreeMap::from([(
                            syntax.as_str(),
                            BTreeMap::from([(target_name, (doc, content))]),
                        )])
                    });
            }
        }

        mod_map
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
            is_module: setting.is_module(),
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
}

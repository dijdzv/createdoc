use super::Setting;
use std::collections::BTreeMap;

type Syntax = String;
type TargetName = String;
type Doc = Vec<String>;
type Content = Vec<String>;
type Filename = String;
type FileVec = Vec<(Syntax, TargetName, Doc, Content)>;
type SyntaxMap<'a> = BTreeMap<&'a str, BTreeMap<&'a TargetName, (&'a Doc, &'a Content)>>;
pub type FileMap<'a> = BTreeMap<&'a Filename, SyntaxMap<'a>>;

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
            cmt_start: setting.cmt().to_owned(),
            target_list: setting.combine_modifier_and_target_list(),
            is_doc: false,
            is_content: false,
            read_dir: setting.read_dir().to_owned(),
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

use super::Setting;
use std::collections::BTreeMap;

type Syntax = String;
type TargetName = String;
type Doc = Vec<String>;
type Content = Vec<String>;
type Filename = String;
type FileVec = Vec<(Syntax, TargetName, Doc, Content)>;
type SyntaxMap = BTreeMap<Syntax, BTreeMap<TargetName, (Doc, Content)>>;
pub type FileMap = BTreeMap<Filename, SyntaxMap>;

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
    pub fn file_and_syntax_categorize(&self) -> FileMap {
        let mut mod_map: FileMap = BTreeMap::new();
        for (filename, file_vec) in &self.all {
            for (syntax, target_name, doc, content) in file_vec {
                mod_map
                    .entry(filename.to_owned())
                    .and_modify(|e| {
                        e.entry(syntax.to_owned())
                            .and_modify(|e| {
                                e.insert(
                                    target_name.to_owned(),
                                    (doc.to_owned(), content.to_owned()),
                                );
                            })
                            .or_insert_with(|| {
                                BTreeMap::from([(
                                    target_name.to_owned(),
                                    (doc.to_owned(), content.to_owned()),
                                )])
                            });
                    })
                    .or_insert_with(|| {
                        BTreeMap::from([(
                            syntax.to_owned(),
                            BTreeMap::from([(
                                target_name.to_owned(),
                                (doc.to_owned(), content.to_owned()),
                            )]),
                        )])
                    });
            }
        }
        mod_map

        // self.all.iter().map(|(filename,file_vec)|(filename,)).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _file名とsyntax名毎にカテゴライズ() {
        let all = vec![(
            "add".to_string(),
            vec![(
                "function".to_string(),
                "add_num".to_string(),
                vec!["add_numのDoc".to_string()],
                vec!["add_numのcontents".to_string()],
            )],
        )];
        let categorized = BTreeMap::from([(
            "add".to_string(),
            vec![(
                "function".to_string(),
                "add_num".to_string(),
                vec!["add_numのDoc".to_string()],
                vec!["add_numのcontents".to_string()],
            )],
        )]);
        let read_data = ReadData {
            line: String::new(),
            doc: Vec::new(),
            content: Vec::new(),
            target_name: String::new(),
            syntax: String::new(),
            file_vec: Vec::new(),
            all,
            cmt_start: String::new(),
            target_list: BTreeMap::new(),
            is_doc: false,
            is_content: false,
            read_dir: String::new(),
            is_module: false,
        };

        // assert_eq!(read_data.file_and_syntax_categorize(), categorized);
    }
}

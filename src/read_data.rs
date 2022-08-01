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
                    .entry(filename.clone())
                    .and_modify(|e| {
                        e.entry(syntax.clone())
                            .and_modify(|e| {
                                e.insert(target_name.clone(), (doc.clone(), content.clone()));
                            })
                            .or_insert_with(|| {
                                BTreeMap::from([(
                                    target_name.clone(),
                                    (doc.clone(), content.clone()),
                                )])
                            });
                    })
                    .or_insert_with(|| {
                        BTreeMap::from([(
                            syntax.clone(),
                            BTreeMap::from([(target_name.clone(), (doc.clone(), content.clone()))]),
                        )])
                    });
            }
        }
        mod_map

        // * 抽象化できず
        // self.all
        //     .iter()
        //     .map(|(filename, file_vec)| {
        //         (
        //             filename.clone(),
        //             file_vec
        //                 .iter()
        //                 .map(|(syntax, target_name, doc, content)| {
        //                     (
        //                         syntax.clone(),
        //                         (target_name.clone(), (doc.clone(), content.clone())),
        //                     )
        //                 })
        //                 .collect(),
        //         )
        //     })
        //     .collect()
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
        self.all.push((filename, self.file_vec.clone()));
    }
    pub fn push_content(&mut self) {
        self.content.push(self.line.clone());
    }
    pub fn push_doc(&mut self) {
        self.doc.push(self.line.clone());
    }
    pub fn push_file_vec(&mut self) {
        self.file_vec.push((
            self.syntax.clone(),
            self.target_name.clone(),
            self.doc.clone(),
            self.content.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _file名とsyntax名毎にカテゴライズ() {
        let all = vec![
            (
                "add".to_string(),
                vec![
                    (
                        "fn".to_string(),
                        "add_int".to_string(),
                        vec!["add_intのDoc".to_string()],
                        vec!["add_intのContent".to_string()],
                    ),
                    // syntaxが同じ
                    (
                        "fn".to_string(),
                        "add_float".to_string(),
                        vec!["add_floatのDoc".to_string()],
                        vec!["add_floatのContent".to_string()],
                    ),
                    // syntaxが異なる
                    (
                        "struct".to_string(),
                        "Add".to_string(),
                        vec!["AddのDoc".to_string()],
                        vec!["AddのContent".to_string()],
                    ),
                ],
            ),
            // fileが同じ
            (
                "add".to_string(),
                vec![
                    // syntaxが同じ
                    (
                        "fn".to_string(),
                        "add_int".to_string(),
                        vec!["add_intのDoc".to_string()],
                        vec!["add_intのContent".to_string()],
                    ),
                    // syntaxが異なる
                    (
                        "impl".to_string(),
                        "Add".to_string(),
                        vec!["AddのDoc".to_string()],
                        vec!["AddのContent".to_string()],
                    ),
                ],
            ),
            // fileが異なる
            (
                "sub".to_string(),
                vec![(
                    "fn".to_string(),
                    "sub_int".to_string(),
                    vec!["sub_intのDoc".to_string()],
                    vec!["sub_intのContent".to_string()],
                )],
            ),
        ];
        let categorized = BTreeMap::from([
            (
                "add".to_string(),
                BTreeMap::from([
                    (
                        "fn".to_string(),
                        BTreeMap::from([
                            (
                                "add_int".to_string(),
                                (
                                    vec!["add_intのDoc".to_string()],
                                    vec!["add_intのContent".to_string()],
                                ),
                            ),
                            (
                                "add_float".to_string(),
                                (
                                    vec!["add_floatのDoc".to_string()],
                                    vec!["add_floatのContent".to_string()],
                                ),
                            ),
                        ]),
                    ),
                    (
                        "struct".to_string(),
                        BTreeMap::from([(
                            "Add".to_string(),
                            (
                                vec!["AddのDoc".to_string()],
                                vec!["AddのContent".to_string()],
                            ),
                        )]),
                    ),
                    (
                        "impl".to_string(),
                        BTreeMap::from([(
                            "Add".to_string(),
                            (
                                vec!["AddのDoc".to_string()],
                                vec!["AddのContent".to_string()],
                            ),
                        )]),
                    ),
                ]),
            ),
            (
                "sub".to_string(),
                BTreeMap::from([(
                    "fn".to_string(),
                    BTreeMap::from([(
                        "sub_int".to_string(),
                        (
                            vec!["sub_intのDoc".to_string()],
                            vec!["sub_intのContent".to_string()],
                        ),
                    )]),
                )]),
            ),
        ]);
        let read_data = ReadData {
            all,
            // ↓使わない
            line: String::new(),
            doc: Vec::new(),
            content: Vec::new(),
            target_name: String::new(),
            syntax: String::new(),
            file_vec: Vec::new(),
            cmt_start: String::new(),
            target_list: BTreeMap::new(),
            is_doc: false,
            is_content: false,
            read_dir: String::new(),
            is_module: false,
        };

        assert_eq!(read_data.file_and_syntax_categorize(), categorized);
    }
}

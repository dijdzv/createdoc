use serde_derive::*;
use std::iter;
use std::{collections::BTreeMap, path::Path};

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
    pub fn cmt(&self) -> &str {
        &self.read.cmt_start
    }
    pub fn combine_modifier_and_target_list(&self) -> BTreeMap<String, Vec<String>> {
        self.read
            .target_list
            .iter()
            .map(|target_name| {
                (
                    target_name.to_owned(),
                    self.read
                        .modifier
                        .iter()
                        .map(|modifier| format!("{} {}", modifier, target_name))
                        .chain(iter::once(target_name.to_owned()))
                        .collect(),
                )
            })
            .collect()
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

use std::{collections::HashMap, fs::File, io::Write};

pub fn search_result(file: &mut File, search_data: HashMap<&str, Vec<&str>>) {
    for (k, v) in search_data {
        file.write_all(format!("<>").as_bytes()).unwrap();
    }
}

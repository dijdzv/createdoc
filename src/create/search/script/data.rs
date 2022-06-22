use crate::FolderVec;
use std::collections::HashMap;

pub fn search_data(folder_vec: &FolderVec) -> String {
    let mut hashmap = HashMap::new();

    for (filename, file_vec) in folder_vec {
        let target_names = file_vec
            .iter()
            .map(|(t, _, _)| t.as_str())
            .collect::<Vec<_>>();

        hashmap.insert(&filename[..filename.find('.').unwrap()], target_names);
    }
    let mut buf = Vec::new();
    for (k, v) in hashmap {
        buf.push(format!("{}: [{}]", k, v.join(",")));
    }
    let hash_str = buf.join(",\n");

    format!("const search_data = {{\n {}\n }}", hash_str)
}

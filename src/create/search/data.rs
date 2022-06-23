use crate::FolderVec;
use std::collections::HashMap;

pub fn search_data(folder_vec: &FolderVec) -> HashMap<&str, Vec<&str>> {
    let mut hashmap = HashMap::new();

    for (filename, file_vec) in folder_vec {
        let target_names = file_vec
            .iter()
            .map(|(t, _, _)| t.as_str())
            .collect::<Vec<_>>();

        hashmap.insert(&filename[..filename.find('.').unwrap()], target_names);
    }

    hashmap
}

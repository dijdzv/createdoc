use crate::FolderVec;
use std::collections::HashMap;

pub fn search_data(folder_vec: &FolderVec) -> Vec<(&str, Vec<&str>)> {
    let mut hashmap = HashMap::new();

    for (filename, file_vec) in folder_vec {
        let target_names = file_vec
            .iter()
            .map(|(t, _, _)| t.as_str())
            .collect::<Vec<_>>();

        hashmap.insert(&filename[..filename.find('.').unwrap()], target_names);
    }

    let mut search_data = hashmap.into_iter().collect::<Vec<_>>();
    search_data.sort_by(|a, b| a.0.cmp(b.0));
    search_data
}

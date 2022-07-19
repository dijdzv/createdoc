use crate::error::ErrorMsg;
use createdoc::DirVec;

use anyhow::anyhow;
use std::{collections::HashMap, path::Path};

pub fn search_data(dir_vec: &DirVec) -> anyhow::Result<Vec<(&str, Vec<&str>)>> {
    let mut hashmap = HashMap::new();

    for (filename, file_vec) in dir_vec {
        let target_names = file_vec
            .iter()
            .map(|(t, _, _)| t.as_str())
            .collect::<Vec<_>>();
        let stem_name = Path::new(filename)
            .file_stem()
            .ok_or_else(|| anyhow!(ErrorMsg::FileStem.as_str()))?
            .to_str()
            .ok_or_else(|| anyhow!(ErrorMsg::ToStr.as_str()))?;

        hashmap.insert(stem_name, target_names);
    }

    let mut search_data = hashmap.into_iter().collect::<Vec<_>>();
    search_data.sort_by(|a, b| a.0.cmp(b.0));
    Ok(search_data)
}

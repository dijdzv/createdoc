use crate::error::ErrorMsg;
use createdoc::{All, AllVec};

use anyhow::Context;
use std::{collections::HashMap, path::Path};

type SyntaxAndTarget<'a> = Vec<(&'a String, &'a String)>;

pub fn search_data<'a>(
    all: &'a AllVec,
    categorized: &'a All,
) -> anyhow::Result<Vec<(&'a str, SyntaxAndTarget<'a>)>> {
    let mut hashmap = HashMap::new();

    for (filename, file_vec) in all {
        let syntax_and_target = file_vec
            .iter()
            .map(|(s, t, _, _)| (s, t))
            .collect::<Vec<_>>();
        let stem_name = Path::new(filename)
            .file_stem()
            .with_context(|| ErrorMsg::FileStem.as_str())?
            .to_str()
            .with_context(|| ErrorMsg::ToStr.as_str())?;

        hashmap.insert(stem_name, syntax_and_target);
    }

    // for (filename, syntax_vec) in categorized {
    //     for (syntax, target) in syntax_vec {

    //     }
    // }

    let mut search_data = hashmap.into_iter().collect::<Vec<_>>();
    search_data.sort_by(|a, b| a.0.cmp(b.0));
    Ok(search_data)
}

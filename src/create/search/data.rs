use createdoc::{All, AllVec};

use std::collections::HashMap;

type SyntaxAndTarget<'a> = Vec<(&'a String, Vec<&'a String>)>;

pub fn search_data<'a>(
    all: &'a AllVec,
    categorized: &'a All,
) -> anyhow::Result<Vec<(&'a String, SyntaxAndTarget<'a>)>> {
    let mut hashmap = HashMap::new();

    for (filename, syntax_vec) in categorized {
        let syntax_and_target = syntax_vec
            .iter()
            .map(|(s, t)| (*s, t.iter().map(|t| t.0).collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        hashmap.insert(*filename, syntax_and_target);
    }

    let mut search_data = hashmap.into_iter().collect::<Vec<_>>();
    search_data.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(search_data)
}

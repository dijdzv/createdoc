use crate::FileMap;

use std::collections::HashMap;

pub type SyntaxAndTarget<'a> = Vec<(&'a str, Vec<&'a String>)>;

pub fn search_data<'a>(
    file_map: &'a FileMap,
) -> anyhow::Result<Vec<(&'a String, SyntaxAndTarget<'a>)>> {
    let mut hashmap = HashMap::new();

    for (filename, syntax_map) in file_map {
        let syntax_and_target = syntax_map
            .iter()
            .map(|(s, t)| (*s, t.iter().map(|(&t, _)| t).collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        hashmap.insert(*filename, syntax_and_target);
    }

    let mut search_data = hashmap.into_iter().collect::<Vec<_>>();
    search_data.sort_by(|a, b| a.0.cmp(b.0));
    Ok(search_data)
}

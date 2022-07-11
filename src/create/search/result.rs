use std::{collections::HashMap, fs::File, io::Write};

pub fn search_result(file: &mut File, search_data: &HashMap<&str, Vec<&str>>) {
    file.write_all(r#"<div class="search-result"><ul>"#.as_bytes())
        .unwrap();
    for (filename, v) in search_data {
        file.write_all(
            format!(
                "<a href=\"#{}\"><li class=\"search-list dn\">{}</li></a>",
                filename, filename
            )
            .as_bytes(),
        )
        .unwrap();

        for target_name in v {
            file.write_all(
                format!(
                    "<a href=\"#{}\"><li class=\"search-list dn\">{}<span class=\"s-target_name\">{}</span></li></a>",
                    target_name, filename, target_name
                )
                .as_bytes(),
            )
            .unwrap();
        }
    }
    file.write_all("</ul></div>".as_bytes()).unwrap();
}

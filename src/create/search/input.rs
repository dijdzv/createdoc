use std::{fs::File, io::Write};

pub fn input(file: &mut File) {
    file.write_all(
        format!(
            "{}{}{}{}",
            r#"<form id="search" role="search">"#,
            r#"<input type="search" id="search-input" name="search" spellcheck="false" autocomplete="off"
            placeholder="Click or press `S` to search">"#,
            r#"<button type="submit">search</button>"#,
            "</form>"
        )
        .as_bytes(),
    )
    .unwrap();
}

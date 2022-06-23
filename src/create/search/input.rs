use std::{fs::File, io::Write};

pub fn search_input(file: &mut File) {
    file.write_all(
      r#"<form id="search" role="search" onsubmit="return false;"><input type="search" id="search-input" name="search" spellcheck="false" autocomplete="off"
      placeholder="Click or press `S` to search"><button type="button"><i class="gg-search"></i></button></form>"#
      .as_bytes(),
    )
    .unwrap();
}

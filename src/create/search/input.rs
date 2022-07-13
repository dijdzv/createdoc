use std::{fs::File, io::Write};

pub fn search_input(file: &mut File) -> Result<(), std::io::Error> {
    file.write_all(
      r#"<form id="search" role="search" onsubmit="return false;"><input type="search" id="search-input" name="search" spellcheck="false" autocomplete="off"
      placeholder="Click or press `S` to search"></form>"#
      .as_bytes(),
    )?;

    Ok(())
}

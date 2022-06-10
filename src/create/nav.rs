use crate::create::constant;
use std::{fs::File, io::Write};

pub fn create_nav(file: &mut File, file_vec: &[Vec<Vec<String>>]) {
    // nav
    file.write_all(r#"<div class="nav">"#.as_bytes()).unwrap();

    // /nav
    file.write_all(r#"</div>"#.as_bytes()).unwrap();
}

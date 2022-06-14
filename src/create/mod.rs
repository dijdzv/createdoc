use super::FolderVec;
use std::{fs::File, io::Write};

mod constant;
mod main;
mod nav;

/// htmlファイルを生成
pub fn create_html(create_dir: &str, create_filename: &mut String, folder_vec: &FolderVec) {
    if create_filename.contains('.') {
        let cp = create_filename.find('.').unwrap();
        *create_filename = create_filename[..cp].to_string();
    }
    let create_filepath = create_dir.to_string() + create_filename;

    let mut file = File::create(create_filepath + ".html").unwrap();
    // html
    file.write_all(constant::HTML_START.as_bytes()).unwrap();

    // wrap
    file.write_all(r#"<div class="wrap">"#.as_bytes()).unwrap();

    nav::create_nav(&mut file, folder_vec, create_filename);

    main::create_main(&mut file, folder_vec);

    // /wrap
    file.write_all("</div>".as_bytes()).unwrap();

    // /html
    file.write_all(constant::HTML_END.as_bytes()).unwrap();
}

use super::FolderVec;
use std::{fs::File, io::Write};

mod constant;
mod main;
mod nav;
mod search;

/// htmlファイルを生成
pub fn create_html(create_dir: &str, read_lang: &str, folder_vec: &FolderVec) {
    let create_filepath = create_dir.to_string() + read_lang + "doc";

    let mut file = File::create(create_filepath + ".html").unwrap();
    // html
    file.write_all(constant::HTML_TOP_START.as_bytes()).unwrap();
    file.write_all(constant::STYLE.as_bytes()).unwrap();
    file.write_all(constant::PRISM_CSS.as_bytes()).unwrap();
    file.write_all(constant::HTML_TOP_END.as_bytes()).unwrap();

    // wrap
    file.write_all(r#"<div class="wrap">"#.as_bytes()).unwrap();

    nav::create_nav(&mut file, folder_vec, read_lang);

    main::create_main(&mut file, folder_vec, read_lang);

    // /wrap
    file.write_all("</div>".as_bytes()).unwrap();

    // script
    file.write_all(constant::SCRIPT.as_bytes()).unwrap();
    file.write_all(constant::PRISM_JS.as_bytes()).unwrap();
    // search script
    let search_data = search::search_data(folder_vec);
    println!("{}", search_data);

    // /html
    file.write_all(constant::HTML_BOTTOM.as_bytes()).unwrap();
}

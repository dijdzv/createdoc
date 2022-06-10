use std::{fs::File, io::Write};
mod constant;
mod main;
mod nav;

/// htmlファイルを生成
pub fn create_html(file_vec: &mut [Vec<Vec<String>>]) {
    let mut file = File::create("rsdoc.html").unwrap();
    // html
    file.write_all(constant::HTML_START.as_bytes()).unwrap();

    // wrap
    file.write_all(r#"<div class="wrap">"#.as_bytes()).unwrap();

    nav::create_nav(&mut file, file_vec);

    main::create_main(&mut file, file_vec);

    // /wrap
    file.write_all("</div>".as_bytes()).unwrap();

    // /html
    file.write_all(constant::HTML_END.as_bytes()).unwrap();
}

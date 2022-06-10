use std::{fs::File, io::Write};

use chrono::Local;

pub fn create_nav(file: &mut File, file_vec: &Vec<(String, Vec<String>, Vec<String>)>) {
    // nav
    file.write_all(r#"<nav>"#.as_bytes()).unwrap();

    file.write_all(r#"<ul class="file">"#.as_bytes()).unwrap();
    file.write_all(r#"<li id="csv">"#.as_bytes()).unwrap();
    file.write_all("csv -".as_bytes()).unwrap();
    file.write_all("</li>".as_bytes()).unwrap();

    file.write_all(r#"<ul class="func csv">"#.as_bytes())
        .unwrap();

    for (name, _, _) in file_vec {
        file.write_all((r##"<a href="#"##.to_string() + name + r#""><li>"#).as_bytes())
            .unwrap();
        file.write_all(name.as_bytes()).unwrap();
        file.write_all("</li></a>".as_bytes()).unwrap();
    }
    // /func
    file.write_all("</ul>".as_bytes()).unwrap();

    // /file
    file.write_all("</ul>".as_bytes()).unwrap();

    file.write_all("<footer>".as_bytes()).unwrap();
    let now = Local::now().format("%Y年%m月%d日 %H:%M:%S").to_string();
    file.write_all(now.as_bytes()).unwrap();
    file.write_all("</footer>".as_bytes()).unwrap();

    // /nav
    file.write_all("</nav>".as_bytes()).unwrap();
}

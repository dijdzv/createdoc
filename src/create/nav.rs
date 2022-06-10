use std::{fs::File, io::Write};

pub fn create_nav(file: &mut File, file_vec: &Vec<(String, Vec<String>, Vec<String>)>) {
    // nav
    file.write_all(r#"<nav>"#.as_bytes()).unwrap();

    file.write_all("<li>".as_bytes()).unwrap();
    file.write_all("csv".as_bytes()).unwrap();
    file.write_all("</li>".as_bytes()).unwrap();

    file.write_all(r#"<ul class="func csv">"#.as_bytes())
        .unwrap();

    for (name, _, _) in file_vec {
        file.write_all((r##"<a href="#"##.to_string() + name + r#""><li>"#).as_bytes())
            .unwrap();
        file.write_all(name.as_bytes()).unwrap();
        file.write_all("</li></a>".as_bytes()).unwrap();
    }
    file.write_all("</ul>".as_bytes()).unwrap();

    // /nav
    file.write_all("</nav>".as_bytes()).unwrap();
}

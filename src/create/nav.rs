use chrono::Local;
use std::{fs::File, io::Write};

pub fn create_nav(
    file: &mut File,
    folder_vec: &Vec<(&String, Vec<(String, Vec<String>, Vec<String>)>)>,
) {
    // nav
    file.write_all(r#"<nav>"#.as_bytes()).unwrap();

    //  n-folder
    file.write_all(r#"<div class="n-folder">"#.as_bytes())
        .unwrap();
    for (filename, file_vec) in folder_vec {
        // n-file
        file.write_all(
            (r#"<h4 class="n-file" id="n-"#.to_string() + filename + r#"">"#).as_bytes(),
        )
        .unwrap();
        file.write_all((filename.to_string() + " -").as_bytes())
            .unwrap();
        // /n-file
        file.write_all("</h4>".as_bytes()).unwrap();

        //func
        file.write_all((r#"<ul class="n-func n-"#.to_string() + filename + r#" dn">"#).as_bytes())
            .unwrap();

        for (name, _, _) in file_vec {
            file.write_all((r##"<a href="#"##.to_string() + name + r#""><li>"#).as_bytes())
                .unwrap();
            file.write_all(name.as_bytes()).unwrap();
            file.write_all("</li></a>".as_bytes()).unwrap();
        }
        // /func
        file.write_all("</ul>".as_bytes()).unwrap();
    }

    // /n-folder
    file.write_all("</div>".as_bytes()).unwrap();

    // footer
    file.write_all("<footer>".as_bytes()).unwrap();
    let now = Local::now().format("%Y年%m月%d日 %H:%M:%S").to_string();
    file.write_all(now.as_bytes()).unwrap();
    file.write_all("</footer>".as_bytes()).unwrap();

    // /nav
    file.write_all("</nav>".as_bytes()).unwrap();
}

use super::FolderVec;
use chrono::Local;
use std::{fs::File, io::Write};

pub fn create_nav(file: &mut File, folder_vec: &FolderVec) {
    // nav
    file.write_all(r#"<nav>"#.as_bytes()).unwrap();

    // category
    file.write_all(r#"<h2 class="category">"#.as_bytes())
        .unwrap();
    file.write_all("function".as_bytes()).unwrap();
    file.write_all("</h2>".as_bytes()).unwrap();

    //  n-folder
    file.write_all(r#"<div class="n-folder">"#.as_bytes())
        .unwrap();
    for (filename, file_vec) in folder_vec {
        // n-file
        file.write_all(r#"<div class="n-file">"#.as_bytes())
            .unwrap();
        // n-filename
        file.write_all(
            (r#"<h3 class="n-filename" id="n-"#.to_string() + filename + r#"">"#).as_bytes(),
        )
        .unwrap();
        let show_name = if filename.contains('.') {
            let cp = filename.find('.').unwrap();
            filename[..cp].to_string()
        } else {
            filename.to_string()
        };
        file.write_all(show_name.to_string().as_bytes()).unwrap();
        // /n-filename
        file.write_all("</h3>".as_bytes()).unwrap();

        // n-func
        file.write_all((r#"<ul class="n-func n-"#.to_string() + filename + r#" dn">"#).as_bytes())
            .unwrap();

        for (name, _, _) in file_vec {
            file.write_all((r##"<a href="#"##.to_string() + name + r#""><li>"#).as_bytes())
                .unwrap();
            file.write_all(name.as_bytes()).unwrap();
            file.write_all("</li></a>".as_bytes()).unwrap();
        }
        // /n-func
        file.write_all("</ul>".as_bytes()).unwrap();

        // n-file
        file.write_all("</div>".as_bytes()).unwrap();
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

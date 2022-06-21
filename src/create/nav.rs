use super::FolderVec;
use chrono::Local;
use std::{fs::File, io::Write};

pub fn create_nav(file: &mut File, folder_vec: &FolderVec, read_lang: &str) {
    // nav
    file.write_all(r#"<nav>"#.as_bytes()).unwrap();

    // category
    file.write_all(r#"<a href=""><h2 class="html-filename">"#.as_bytes())
        .unwrap();
    file.write_all(format!("{}{}", read_lang, "doc").as_bytes())
        .unwrap();
    file.write_all("</h2></a>".as_bytes()).unwrap();

    //  n-folder
    file.write_all(r#"<div class="n-folder">"#.as_bytes())
        .unwrap();
    for (filename, file_vec) in folder_vec {
        // n-file
        file.write_all(r#"<div class="n-file">"#.as_bytes())
            .unwrap();
        // n-filename
        let show_name = if filename.contains('.') {
            let cp = filename.find('.').unwrap();
            filename[..cp].to_string()
        } else {
            filename.to_string()
        };
        file.write_all(
            format!(
                "{}{}{}{}{}",
                r#"<h3 class="n-filename" id="n-"#, filename, r#"">"#, show_name, "</h3>"
            )
            .as_bytes(),
        )
        .unwrap();

        // n-syntax
        file.write_all(
            format!("{}{}{}", r#"<ul class="n-target n-"#, filename, r#" dn">"#).as_bytes(),
        )
        .unwrap();

        // li
        for (target_name, _, _) in file_vec {
            file.write_all(
                format!(
                    "{}{}{}{}{}",
                    r##"<a href="#"##, target_name, r#""><li>"#, target_name, "</li></a>"
                )
                .as_bytes(),
            )
            .unwrap();
        }

        // /n-syntax
        file.write_all("</ul>".as_bytes()).unwrap();

        // n-file
        file.write_all("</div>".as_bytes()).unwrap();
    }

    // /n-folder
    file.write_all("</div>".as_bytes()).unwrap();

    // footer
    let now = Local::now().format("%Y年%m月%d日 %H:%M:%S").to_string();
    file.write_all(
        format!(
            "{}{}{}",
            r#"<div class="bottom"><p class="time">"#, now, "</p></div>"
        )
        .as_bytes(),
    )
    .unwrap();

    // /nav
    file.write_all("</nav>".as_bytes()).unwrap();
}

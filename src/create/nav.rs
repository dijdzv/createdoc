use super::FolderVec;
use chrono::Local;
use std::{fs::File, io::Write};

pub fn create_nav(file: &mut File, folder_vec: &FolderVec, read_lang: &str) {
    // nav
    file.write_all("<nav>".as_bytes()).unwrap();

    // category
    file.write_all(r#"<a href=""><h2 class="html-filename">"#.as_bytes())
        .unwrap();
    file.write_all(format!("{}doc", read_lang).as_bytes())
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
        let cp = filename.find('.').unwrap();
        let show_name = filename[..cp].to_string();
        file.write_all(
            format!(
                "<h3 class=\"n-filename\" id=\"n-{}\">{}</h3>",
                filename, show_name
            )
            .as_bytes(),
        )
        .unwrap();

        // n-syntax
        file.write_all(format!("<ul class=\"n-target n-{} dn\">", filename).as_bytes())
            .unwrap();

        // li
        for (target_name, _, _) in file_vec {
            file.write_all(
                format!("<a href=\"#{}\"><li>{}</li></a>", target_name, target_name).as_bytes(),
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
    let now = Local::now().format("%Y/%m/%d\n%H:%M:%S").to_string();
    file.write_all(format!("<div class=\"bottom\"><p class=\"time\">{}</p></div>", now).as_bytes())
        .unwrap();

    // /nav
    file.write_all("</nav>".as_bytes()).unwrap();
}

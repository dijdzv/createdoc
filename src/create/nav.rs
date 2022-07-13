use super::FolderVec;
use chrono::Local;
use std::{fs::File, io::Write};

pub fn create_nav(
    file: &mut File,
    folder_vec: &FolderVec,
    read_lang: &str,
) -> Result<(), std::io::Error> {
    // nav
    file.write_all("<nav>".as_bytes())?;

    // category
    file.write_all(r#"<a href=""><h2 class="html-filename">"#.as_bytes())?;
    file.write_all(format!("{}doc", read_lang).as_bytes())?;
    file.write_all("</h2></a>".as_bytes())?;

    //  n-folder
    file.write_all(r#"<div class="n-folder">"#.as_bytes())?;
    for (filename, file_vec) in folder_vec {
        // n-file
        file.write_all(r#"<div class="n-file">"#.as_bytes())?;
        // n-filename
        let cp = filename.find('.').unwrap();
        let show_name = filename[..cp].to_string();
        file.write_all(
            format!(
                "<a href=\"#f-{}\"><h4 class=\"n-filename\" id=\"n-{}\">{}</h4></a>",
                show_name, filename, show_name
            )
            .as_bytes(),
        )?;

        // n-target
        file.write_all(format!("<ul class=\"n-target n-{}\">", filename).as_bytes())?;

        // li
        for (target_name, _, _) in file_vec {
            file.write_all(
                format!(
                    "<a href=\"#t-{}\"><li>{}</li></a>",
                    target_name, target_name
                )
                .as_bytes(),
            )?;
        }

        // /n-syntax
        file.write_all("</ul>".as_bytes())?;

        // n-file
        file.write_all("</div>".as_bytes())?;
    }

    // /n-folder
    file.write_all("</div>".as_bytes())?;

    // footer
    let now = Local::now().format("%Y/%m/%d\n%H:%M:%S").to_string();
    file.write_all(
        format!("<div class=\"bottom\"><p class=\"time\">{}</p></div>", now).as_bytes(),
    )?;

    // /nav
    file.write_all("</nav>".as_bytes())?;

    Ok(())
}

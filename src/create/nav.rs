use crate::error::ErrorMsg;
use crate::FolderVec;

use chrono::Local;
use createdoc::Output;
use std::path::Path;

pub fn create_nav(
    output: &mut Output,
    folder_vec: &FolderVec,
    read_lang: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // nav
    output.add("<nav>");

    // category
    output.add(r#"<a href=""><h2 class="html-filename">"#);
    output.add(format!("{}doc", read_lang));
    output.add("</h2></a>");

    //  n-folder
    output.add(r#"<div class="n-folder">"#);
    for (filename, file_vec) in folder_vec {
        // n-file
        output.add(r#"<div class="n-file">"#);
        // n-filename
        let stem_name = Path::new(filename)
            .file_stem()
            .ok_or_else(|| ErrorMsg::FileStem.as_str())?
            .to_str()
            .ok_or_else(|| ErrorMsg::ToStr.as_str())?;

        output.add(format!(
            "<a href=\"#f-{}\"><h4 class=\"n-filename\" id=\"n-{}\">{}</h4></a>",
            stem_name, filename, stem_name
        ));

        // n-target
        output.add(format!("<ul class=\"n-target n-{}\">", filename));

        // li
        for (target_name, _, _) in file_vec {
            output.add(format!(
                "<a href=\"#t-{}\"><li>{}</li></a>",
                target_name, target_name
            ));
        }

        // /n-syntax
        output.add("</ul>");

        // n-file
        output.add("</div>");
    }

    // /n-folder
    output.add("</div>");

    // footer
    let now = Local::now().format("%Y/%m/%d\n%H:%M:%S").to_string();
    output.add(format!(
        "<div class=\"bottom\"><p class=\"time\">{}</p></div>",
        now
    ));

    // /nav
    output.add("</nav>");

    Ok(())
}

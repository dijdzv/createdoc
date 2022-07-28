use createdoc::{FileMap, Output};

use chrono::Local;

pub fn create_nav(output: &mut Output, file_map: &FileMap, read_lang: &str) -> anyhow::Result<()> {
    // nav
    output.add("<nav>");

    // top
    output.add(r#"<a href=""><h2 class="html-filename">"#);
    output.add(format!("{}doc", read_lang));
    output.add("</h2></a>");

    //  n-folder
    output.add(r#"<div class="n-folder">"#);
    for (filename, syntax_map) in file_map {
        // n-file
        output.add(r#"<div class="n-file">"#);

        // n-filename
        output.add(format!(
            "<a href=\"#f-{}\"><h2 class=\"n-filename\" id=\"n-{0}\">{0}</h2></a>",
            filename
        ));
        for (syntax, target_map) in syntax_map {
            output.add(format!("<h3 class=\"n-syntax\">{}</h3>", syntax));

            for target_name in target_map.keys() {
                // n-target
                output.add(format!("<ul class=\"n-target n-{}\">", filename));

                // li
                output.add(format!(
                    "<a href=\"#t-{}-{}\"><li>{}</li></a>",
                    syntax, target_name, target_name
                ));

                // /n-syntax
                output.add("</ul>");
            }
        }
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

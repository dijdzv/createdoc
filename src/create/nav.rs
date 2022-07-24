use crate::error::ErrorMsg;
use createdoc::{Categorized, Output};

use anyhow::Context;
use chrono::Local;
use std::path::Path;

pub fn create_nav(
    output: &mut Output,
    categorized: &Categorized,
    read_lang: &str,
) -> anyhow::Result<()> {
    // nav
    output.add("<nav>");

    // top
    output.add(r#"<a href=""><h2 class="html-filename">"#);
    output.add(format!("{}doc", read_lang));
    output.add("</h2></a>");

    //  n-folder
    output.add(r#"<div class="n-folder">"#);
    for (filename, syntax_hash) in categorized {
        // n-file
        output.add(r#"<div class="n-file">"#);

        // n-filename
        let stem_name = Path::new(filename)
            .file_stem()
            .with_context(|| ErrorMsg::FileStem.as_str())?
            .to_str()
            .with_context(|| ErrorMsg::ToStr.as_str())?;

        output.add(format!(
            "<a href=\"#f-{}\"><h2 class=\"n-filename\" id=\"n-{}\">{}</h2></a>",
            stem_name, filename, stem_name
        ));
        for (syntax, target_vec) in syntax_hash {
            output.add(format!("<h3 class=\"n-syntax\">{}</h3>", syntax));

            for (target_name, _, _) in target_vec {
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

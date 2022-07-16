use super::search;
use crate::create::constant;
use crate::error::ErrorMsg;
use crate::FolderVec;

use createdoc::Output;
use regex::Regex;
use std::path::Path;

pub fn create_main(
    output: &mut Output,
    folder_vec: &FolderVec,
    read_lang: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // main
    output.add("<main>");

    // search
    output.add(r#"<div class="search-area">"#);
    search::search_input(output);
    let search_data = search::search_data(folder_vec)?;
    search::search_result(output, &search_data);
    let mut buf = Vec::new();
    for (k, v) in search_data {
        buf.push(format!(
            "\"{}\",\"{}\"",
            k,
            v.iter()
                .map(|f| format!("{} {}", k, f))
                .collect::<Vec<_>>()
                .join("\",\"")
        ));
    }
    output.add(format!(
        "<script>const searchData = [{}]\n</script>",
        buf.join(",")
    ));

    output.add("</div>");

    for (filename, file_vec) in folder_vec {
        // m-file
        output.add(format!("<div class=\"m-file m-{}\">", filename));

        // h2 m-filename
        let stem_name = Path::new(filename)
            .file_stem()
            .ok_or_else(|| ErrorMsg::FileStem.as_str())?
            .to_str()
            .ok_or_else(|| ErrorMsg::ToStr.as_str())?;

        output.add(format!(
            "<h2 class=\"m-filename\" id=\"f-{}\"><a href=\"#f-{}\">{}</a></h2>",
            stem_name, stem_name, stem_name
        ));

        for (target_name, doc, content) in file_vec {
            // .pair
            output.add(r#"<div class="pair">"#);

            // syntax name
            output.add(
                format!(
                    "<h3 class=\"m-target_name\" id=\"t-{}\">
                    <a href=\"#t-{}\">{}</a><input type=\"text\" class=\"hidden-input\" value=\"{}\">
                    <i class=\"gg-copy\"></i><i class=\"gg-check dn\"></i>
                    </h3>",
                    target_name, target_name, target_name, target_name,
                )
            );

            // docコメント
            output.add(r#"<pre class="doc"><p class="doc-p">"#);
            let re_space = Regex::new(r"[\s\t]+")?;
            let re_tag = Regex::new(r"@[a-zA-Z]+")?;
            let re_type = Regex::new(r#"(array|int(eger)?|string|bool(ean)?|void)[^\s]*"#)?;
            for d in doc {
                // 先頭からtrim
                let d = d.trim_start_matches(constant::TRIM_PATTERN);
                // ２つ以上連続する空白を1つに
                let d = re_space.replace_all(d, " ").into_owned();
                // tagをspanで囲う
                let tag = re_tag.find(&d);
                let d = match tag {
                    Some(t) => {
                        format!(
                            "{}<span class=\"tag\">{}</span>{}",
                            &d[..t.start()],
                            &d[t.start()..t.end()],
                            &d[t.end()..]
                        )
                    }
                    None => d,
                };
                let typ = re_type.find(&d);
                let d = match typ {
                    Some(t) => {
                        format!(
                            "{}<span class=\"type\">{}</span>{}",
                            &d[..t.start()],
                            &d[t.start()..t.end()],
                            &d[t.end()..]
                        )
                    }
                    None => d,
                };
                output.add(format!("{}\n", d));
            }

            // /docコメント
            output.add("</p></pre>");

            // pre code
            output.add(format!(
                "<pre class=\"code\"><code class=\"language-{}\">",
                read_lang
            ));
            for c in content {
                output.add(format!("{}\n", c));
            }
            // /code /pre
            output.add("</code></pre>");

            // /.pair
            output.add("</div>");
        }
        // /m-file
        output.add("</div>");
    }

    // /main
    output.add("</main>");

    Ok(())
}

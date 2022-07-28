use super::search;
use crate::create::constant;
use createdoc::{FileMap, Output};

use regex::Regex;

pub fn create_main(output: &mut Output, file_map: &FileMap, read_lang: &str) -> anyhow::Result<()> {
    // main
    output.add("<main>");

    // search
    output.add(r#"<div class="search-area">"#);
    output.add(search::SEARCH_INPUT);
    let search_data = search::search_data(file_map)?;
    search::search_result(output, search_data);
    output.add("</div>");

    for (filename, syntax_map) in file_map {
        // m-file
        output.add(format!("<div class=\"m-file m-{}\">", filename));

        // TODO into key
        let syntax_list = syntax_map
            .iter()
            .map(|k| k.0.as_str())
            .collect::<Vec<_>>()
            .join(" - ");

        output.add(format!(
            "<h2 class=\"m-filename\" id=\"f-{0}\"><a href=\"#f-{0}\">{0}</a><span class=\"m-syntax\">{1}</span></h2>",
            filename, syntax_list
        ));

        for (syntax, target_map) in syntax_map {
            for (target_name, &(doc, content)) in target_map {
                // .pair
                output.add(r#"<div class="pair">"#);

                // target name
                output.add(
                format!(
                    "<h3 class=\"m-target_name\" id=\"t-{0}-{1}\">
                    <a href=\"#t-{0}-{1}\">{1}</a><input type=\"text\" class=\"hidden-input\" value=\"{1}\">
                    <i class=\"gg-copy\"></i><i class=\"gg-check dn\"></i>
                    </h3>",
                    syntax,target_name
                ));

                // docコメント
                output.add(r#"<pre class="doc"><p class="doc-p">"#);
                let re_space = Regex::new(r"[\s\t]+")?;
                let re_tag = Regex::new(r"@[a-zA-Z]+")?;
                let re_type =
                    Regex::new(r#"(array|int(eger)?|float|string|bool(ean)?|void)[^\s]*"#)?;
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
        }
    }

    // /main
    output.add("</main>");

    Ok(())
}

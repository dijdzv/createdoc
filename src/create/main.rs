use super::FolderVec;
use crate::create::constant;
use regex::Regex;
use std::{fs::File, io::Write};

pub fn create_main(file: &mut File, folder_vec: &FolderVec, read_lang: &str) {
    // main
    file.write_all("<main>".as_bytes()).unwrap();

    for (filename, file_vec) in folder_vec {
        // m-file
        file.write_all(format!("{}{}{}", r#"<div class="m-file m-"#, filename, r#"">"#).as_bytes())
            .unwrap();

        // h2 m-filename
        let show_name = if filename.contains('.') {
            let cp = filename.find('.').unwrap();
            filename[..cp].to_string()
        } else {
            filename.to_string()
        };
        file.write_all(
            format!("{}{}{}", r#"<h2 class="m-filename">"#, show_name, "</h2>").as_bytes(),
        )
        .unwrap();

        for (name, doc, func) in file_vec {
            // .pair
            file.write_all(r#"<div class="pair">"#.as_bytes()).unwrap();

            // syntax name
            file.write_all(
                format!(
                    "{}{}{}{}{}{}{}{}",
                    r#"<h3 class="m-syntax_name" id=""#,
                    name,
                    r#"">"#,
                    name,
                    r#"<input type="text" value=""#,
                    name,
                    r#"">"#,
                    r#"<i class="gg-copy"></i><i class="gg-check dn"></i></h3>"#
                )
                .as_bytes(),
            )
            .unwrap();

            // docコメント
            file.write_all(r#"<pre class="doc"><p class="doc-p">"#.as_bytes())
                .unwrap();
            let re_space = Regex::new(r"[\s\t]+").unwrap();
            let re_tag = Regex::new(r"@[a-zA-Z]+").unwrap();
            for d in doc {
                // 先頭からtrim
                let d = d.trim_start_matches(constant::TRIM_PATTERN);
                // ２つ以上連続する空白を1つに
                let d = re_space.replace_all(d, " ");
                // tagをspanで囲う
                let tag = re_tag.find(&d);
                match tag {
                    Some(t) => {}
                    None => {}
                }

                file.write_all((d.to_owned() + "\r\n").as_bytes()).unwrap();
            }
            // /docコメント
            file.write_all("</p></pre>".as_bytes()).unwrap();

            // pre code
            file.write_all(
                format!(
                    "{}{}{}",
                    r#"<pre class="code"><code class="language-"#, read_lang, r#"">"#
                )
                .as_bytes(),
            )
            .unwrap();
            for f in func {
                file.write_all((f.to_owned() + "\r\n").as_bytes()).unwrap();
            }
            // /code /pre
            file.write_all("</code></pre>".as_bytes()).unwrap();

            // /.pair
            file.write_all("</div>".as_bytes()).unwrap();
        }
        // /m-file
        file.write_all("</div>".as_bytes()).unwrap();
    }

    // /main
    file.write_all("</main>".as_bytes()).unwrap();
}

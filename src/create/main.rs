use super::search;
use super::FolderVec;
use crate::create::constant;
use regex::Regex;
use std::{fs::File, io::Write};

pub fn create_main(file: &mut File, folder_vec: &FolderVec, read_lang: &str) {
    // main
    file.write_all("<main>".as_bytes()).unwrap();

    search::input(file);

    for (filename, file_vec) in folder_vec {
        // m-file
        file.write_all(format!("<div class=\"m-file m-{}\">", filename).as_bytes())
            .unwrap();

        // h2 m-filename
        let cp = filename.find('.').unwrap();
        let show_name = filename[..cp].to_string();

        file.write_all(
            format!(
                "<h2 class=\"m-filename\" id=\"{}\"><a href=\"#{}\">{}</a></h2>",
                show_name, show_name, show_name
            )
            .as_bytes(),
        )
        .unwrap();

        for (target_name, doc, content) in file_vec {
            // .pair
            file.write_all(r#"<div class="pair">"#.as_bytes()).unwrap();

            // syntax name
            file.write_all(
                format!(
                    "<h3 class=\"m-target_name\" id=\"{}\">
                    <a href=\"#{}\">{}</a><input type=\"text\" class=\"hidden-input\" value=\"{}\">
                    <i class=\"gg-copy\"></i><i class=\"gg-check dn\"></i>
                    </h3>",
                    target_name, target_name, target_name, target_name,
                )
                .as_bytes(),
            )
            .unwrap();

            // docコメント
            file.write_all(r#"<pre class="doc"><p class="doc-p">"#.as_bytes())
                .unwrap();
            let re_space = Regex::new(r"[\s\t]+").unwrap();
            let re_tag = Regex::new(r"@[a-zA-Z]+").unwrap();
            let re_type = Regex::new(r#"(array|int(eger)?|string|bool(ean)?|void)[^\s]*"#).unwrap();
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
                file.write_all(format!("{}\n", d).as_bytes()).unwrap();
            }

            // /docコメント
            file.write_all("</p></pre>".as_bytes()).unwrap();

            // pre code
            file.write_all(
                format!(
                    "<pre class=\"code\"><code class=\"language-{}\">",
                    read_lang
                )
                .as_bytes(),
            )
            .unwrap();
            for c in content {
                file.write_all(format!("{}\n", c).as_bytes()).unwrap();
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

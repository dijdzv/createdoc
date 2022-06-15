use super::FolderVec;
use crate::create::constant;
use regex::Regex;
use std::{fs::File, io::Write};

pub fn create_main(file: &mut File, folder_vec: &FolderVec, read_lang: &str) {
    // main
    file.write_all("<main>".as_bytes()).unwrap();

    for (filename, file_vec) in folder_vec {
        file.write_all((r#"<div class="m-file m-"#.to_string() + filename + r#"">"#).as_bytes())
            .unwrap();
        file.write_all(r#"<h2 class="m-filename">"#.as_bytes())
            .unwrap();

        let show_name = if filename.contains('.') {
            let cp = filename.find('.').unwrap();
            filename[..cp].to_string()
        } else {
            filename.to_string()
        };
        file.write_all(show_name.as_bytes()).unwrap();
        file.write_all("</h2>".as_bytes()).unwrap();
        for (name, doc, func) in file_vec {
            // .pair
            file.write_all(r#"<div class="pair">"#.as_bytes()).unwrap();

            // func name
            file.write_all(
                (r#"<h3 class="m-syntax_name" id=""#.to_string()
                    + name
                    + r#"">"#
                    + name
                    + r#"<input type="text" value=""#
                    + name
                    + r#"">"#)
                    .as_bytes(),
            )
            .unwrap();
            file.write_all(r#"<i class="gg-copy"></i><i class="gg-check dn"></i></h3>"#.as_bytes())
                .unwrap();
            // docコメント
            file.write_all(r#"<pre class="doc"><p>"#.as_bytes())
                .unwrap();
            let re = Regex::new(r"[\s\t\v]+").unwrap();
            for d in doc {
                let s = re.replace_all(d.trim_start_matches(constant::TRIM_PATTERN), " ");
                file.write_all((s.to_owned() + "\r\n").as_bytes()).unwrap();
            }
            // /docコメント
            file.write_all("</p></pre>".as_bytes()).unwrap();

            // pre code
            file.write_all(
                (r#"<pre class="code"><code class="language-"#.to_owned() + read_lang + r#"">"#)
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

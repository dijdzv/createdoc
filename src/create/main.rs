use crate::create::constant;
use std::{fs::File, io::Write};

pub fn create_main(file: &mut File, file_vec: &Vec<(String, Vec<String>, Vec<String>)>) {
    // main
    file.write_all("<main>".as_bytes()).unwrap();
    for (name, doc, func) in file_vec {
        // .pair
        file.write_all(r#"<div class="pair">"#.as_bytes()).unwrap();

        // func name
        file.write_all("<h3>".as_bytes()).unwrap();
        file.write_all(name.as_bytes()).unwrap();
        file.write_all("</h3>".as_bytes()).unwrap();

        // docコメント
        file.write_all("<pre><p>".as_bytes()).unwrap();
        for d in doc {
            let s = d.trim_start_matches(constant::TRIM_PATTERN);
            file.write_all((s.to_owned() + "\r\n").as_bytes()).unwrap();
        }
        // /docコメント
        file.write_all("</p></pre>".as_bytes()).unwrap();

        // pre code
        file.write_all(r#"<pre class="prettyprint linenums"><code>"#.as_bytes())
            .unwrap();
        for f in func {
            file.write_all((f.to_owned() + "\r\n").as_bytes()).unwrap();
        }
        // /code /pre
        file.write_all("</code></pre>".as_bytes()).unwrap();

        // /.pair
        file.write_all("</>".as_bytes()).unwrap();
    }
    // /main
    file.write_all("</main>".as_bytes()).unwrap();
}

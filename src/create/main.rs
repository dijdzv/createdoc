use crate::create::constant;
use std::{fs::File, io::Write};

pub fn create_main(file: &mut File, file_vec: &[Vec<Vec<String>>]) {
    // main
    file.write_all(r#"<main>"#.as_bytes()).unwrap();
    for pair in file_vec {
        // .pair
        file.write_all(r#"<div class="pair">"#.as_bytes()).unwrap();

        // docコメント
        file.write_all("<pre><p>".as_bytes()).unwrap();
        for d in &pair[0] {
            let s = d.trim_start_matches(constant::TRIM_PATTERN);
            file.write_all((s.to_owned() + "\r\n").as_bytes()).unwrap();
        }
        // /docコメント
        file.write_all("</p></pre>".as_bytes()).unwrap();

        // pre code
        file.write_all(r#"<pre class="prettyprint linenums"><code>"#.as_bytes())
            .unwrap();
        for f in &pair[1] {
            file.write_all((f.to_owned() + "\r\n").as_bytes()).unwrap();
        }
        // /code /pre
        file.write_all("</code></pre>".as_bytes()).unwrap();

        // /.pair
        file.write_all("</div>".as_bytes()).unwrap();
    }
    // /main
    file.write_all(r#"</main>"#.as_bytes()).unwrap();
}

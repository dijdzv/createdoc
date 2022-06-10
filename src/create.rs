use std::{fs::File, io::Write};

const HTML_START: &str = r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
  p,code{
    white-space: pre;
  }
  .pair{
    background-color: cornsilk;
  }
  .code{
    background-color: lavender;
  }
  code{
    background-color: white;
  }
</style>
<title></title>
</head>
<body>
"#;

const HTML_END: &str = r#"
</body>
</html>"#;

const TRIM_PATTERN: [char; 3] = ['/', '*', ' '];

/// htmlファイルを生成
pub fn create_html(file_vec: &mut [Vec<Vec<String>>]) {
    let mut file = File::create("rsdoc.html").ok().unwrap();
    file.write_all(HTML_START.as_bytes()).unwrap();
    for pair in file_vec {
        // .pair
        file.write_all(r#"<div class="pair">"#.as_bytes()).unwrap();

        // docコメント
        file.write_all("<p>".as_bytes()).unwrap();
        for d in &pair[0] {
            let s = d.trim_start_matches(TRIM_PATTERN);
            file.write_all((s.to_owned() + "\r\n").as_bytes()).unwrap();
        }
        // /docコメント
        file.write_all("</p>".as_bytes()).unwrap();

        // .code
        file.write_all(r#"<div class="code">"#.as_bytes()).unwrap();

        // code
        file.write_all("<code>".as_bytes()).unwrap();
        for f in &pair[1] {
            file.write_all((f.to_owned() + "\r\n").as_bytes()).unwrap();
        }
        // /code
        file.write_all("</code>".as_bytes()).unwrap();

        // /.code
        file.write_all("<div>".as_bytes()).unwrap();

        // /.pair
        file.write_all("</div>".as_bytes()).unwrap();
    }
    file.write_all(HTML_END.as_bytes()).unwrap();
}

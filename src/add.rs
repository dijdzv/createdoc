use regex::Regex;

use crate::{Content, Doc, SyntaxName};

/// 関数とDocのvecを生成
pub fn add_line(
    l: &mut String,
    (i, pre): (usize, &mut usize),
    (is_doc, is_content): (&mut bool, &mut bool),
    (doc, content, syntax_name, file_vec): (
        &mut Doc,
        &mut Content,
        &mut SyntaxName,
        &mut Vec<(String, Doc, Content)>,
    ),
    (cmt_start, cmt_end): (&str, &str),
    target: &[String],
) {
    if l.starts_with(cmt_start) {
        doc.push(l.to_string());
        *is_doc = true; // doc start
        return;
    } else if l.starts_with(cmt_end) && *is_doc {
        doc.push(l.to_string());
        *is_doc = false; // doc end
        *pre = i;
        return;
    }

    for t in target {
        if l.starts_with(t) {
            content.push(l.to_string());
            *is_content = true; // content start
            *l = l.replacen(t, "", 1);
            let re = Regex::new(r"\w+").unwrap();
            let cap = re.captures(l).unwrap();
            *syntax_name = cap.get(0).unwrap().as_str().to_string();
            return;
        }
    }
    if l.starts_with('}') && *is_content {
        content.push(l.to_string());
        *is_content = false; // content end
        file_vec.push((syntax_name.to_string(), doc.to_vec(), content.to_vec()));
        doc.clear();
        content.clear();
    }
    if *is_doc {
        doc.push(l.to_string());
    }
    if *is_content {
        content.push(l.to_string());
    }
}

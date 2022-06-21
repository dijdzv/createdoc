use crate::{Content, Doc, SyntaxName};
use regex::Regex;

/// 関数とDocのvecを生成
pub fn add_line(
    l: &mut String,
    (is_doc, is_content): (&mut bool, &mut bool),
    (doc, content, target_name, file_vec): (
        &mut Doc,
        &mut Content,
        &mut SyntaxName,
        &mut Vec<(String, Doc, Content)>,
    ),
    cmt_start: &str,
    target: &[String],
) {
    for t in target {
        if l.starts_with(t) {
            if *is_doc {
                *is_doc = false;
            }
            content.push(l.to_string());
            *is_content = true; // content start
            *l = l.replacen(t, "", 1);
            let re = Regex::new(r"\w+").unwrap();
            let cap = re.captures(l).unwrap();
            *target_name = cap.get(0).unwrap().as_str().to_string();
            return;
        }
    }
    if l.starts_with('}') && *is_content {
        content.push(l.to_string());
        *is_content = false; // content end
        file_vec.push((target_name.to_string(), doc.to_vec(), content.to_vec()));
        doc.clear();
        content.clear();
    } else if *is_content {
        content.push(l.to_string());
    }

    if *is_doc && l.is_empty() {
        *is_doc = false;
        doc.clear();
    } else if *is_doc {
        doc.push(l.to_string());
    } else if l.starts_with(cmt_start) {
        doc.push(l.to_string());
        *is_doc = true; // doc start
    }
}

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
            println!("1:{}", l);
            l.retain(|c| c != ' ');
            *l = l.replacen(t, "", 1);
            println!("2:{}", l);
            let re = Regex::new("").unwrap();
            let e = match l.find('(') {
                Some(e) => e,
                None => l.rfind("").unwrap(),
            };
            *syntax_name = l[0..e].to_string(); // 構文名を取得
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

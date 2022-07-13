use crate::{error::ErrorMsg, Content, Doc, TargetName};
use regex::Regex;

/// 関数とDocのvecを生成
pub fn add_line(
    l: &mut String,
    (is_doc, is_content): (&mut bool, &mut bool),
    (doc, content, target_name, file_vec): (
        &mut Doc,
        &mut Content,
        &mut TargetName,
        &mut Vec<(String, Doc, Content)>,
    ),
    cmt_start: &str,
    target: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    for t in target {
        if l.starts_with(t) {
            if *is_doc {
                *is_doc = false;
            }
            content.push(l.to_string());
            *is_content = true; // content start
            *l = l.replacen(t, "", 1);
            let re = Regex::new(r"\w+")?;
            let cap = re.captures(l).ok_or_else(|| ErrorMsg::Captures.as_str())?;
            *target_name = cap
                .get(0)
                .ok_or_else(|| ErrorMsg::Get.as_str())?
                .as_str()
                .to_string();
            return Ok(());
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
    Ok(())
}

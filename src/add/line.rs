use crate::error::ErrorMsg;
use crate::ReadData;

use anyhow::Context;
use regex::Regex;

/// 関数とDocのvecを生成
pub fn add_line(read_data: &mut ReadData) -> anyhow::Result<()> {
    let line = read_data.line.clone();
    for (k, v) in read_data.target_list.clone().into_iter() {
        for t in &v {
            if line.starts_with(t) {
                read_data.syntax = k;
                if read_data.is_doc {
                    read_data.is_doc = false;
                }
                read_data.push_content();
                read_data.is_content = true; // content start
                read_data.line = line.replacen(t, "", 1);
                let re = Regex::new(r"\w+")?;
                let cap = re
                    .captures(&read_data.line)
                    .with_context(|| ErrorMsg::Captures.as_str())?;
                read_data.target_name = cap
                    .get(0)
                    .with_context(|| ErrorMsg::Get.as_str())?
                    .as_str()
                    .to_string();
                return Ok(());
            }
        }
    }
    if line.starts_with('}') && read_data.is_content {
        read_data.push_content();
        read_data.is_content = false; // content end
        read_data.push_file_vec();
        read_data.clear_doc();
        read_data.clear_content();
    } else if read_data.is_content {
        read_data.push_content();
    }

    if read_data.is_doc && line.is_empty() {
        read_data.is_doc = false;
        read_data.clear_doc();
    } else if read_data.is_doc {
        read_data.push_doc();
    } else if line.starts_with(&read_data.cmt_start) {
        read_data.push_doc();
        read_data.is_doc = true; // doc start
    }
    Ok(())
}

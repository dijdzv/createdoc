use super::data::SyntaxAndTarget;
use crate::Output;

pub fn search_result(output: &mut Output, search_data: Vec<(&String, SyntaxAndTarget)>) {
    output.add(r#"<div class="search-result"><ul>"#);
    for (filename, syntax_and_target) in &search_data {
        output.add(format!(
            "<a href=\"#f-{}\"><li class=\"search-list dn\">{}</li></a>",
            filename, filename
        ));

        for (syntax, target_names) in syntax_and_target {
            for target_name in target_names {
                output.add(
                format!(
                    "<a href=\"#t-{0}-{1}\"><li class=\"search-list dn\">{2}<span class=\"s-syntax\">{0}</span><span class=\"s-target_name\">{1}</span></li></a>",
                   syntax, target_name, filename
                )
            );
            }
        }
    }
    output.add("</ul></div>");

    let mut buf = Vec::new();
    for (filename, syntax_and_target) in &search_data {
        buf.push(format!("\"{}\"", filename));
        for (syntax, target_names) in syntax_and_target {
            for target_name in target_names {
                buf.push(format!("\"{} {} {}\"", filename, syntax, target_name));
            }
        }
    }
    output.add(format!(
        "<script>const searchData = [{}]\n</script>",
        buf.join(",")
    ));
}

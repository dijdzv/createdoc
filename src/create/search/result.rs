use createdoc::Output;

pub fn search_result(output: &mut Output, search_data: &Vec<(&str, Vec<&str>)>) {
    output.add(r#"<div class="search-result"><ul>"#);
    for (filename, v) in search_data {
        output.add(format!(
            "<a href=\"#{}\"><li class=\"search-list dn\">{}</li></a>",
            filename, filename
        ));

        for target_name in v {
            output.add(
                format!(
                    "<a href=\"#{}\"><li class=\"search-list dn\">{}<span class=\"s-target_name\">{}</span></li></a>",
                    target_name, filename, target_name
                )
            );
        }
    }
    output.add("</ul></div>");
}

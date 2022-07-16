use createdoc::Output;

pub fn search_input(output: &mut Output) {
    output.add(
      r#"<form id="search" role="search" onsubmit="return false;"><input type="search" id="search-input" name="search" spellcheck="false" autocomplete="off"
      placeholder="Click or press `S` to search"></form>"#
    );
}

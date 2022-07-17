pub const TOML: &str = r#"[dir]
read_dir = "./"
# Can use `*`
# Specify by name or path
read_folder = [""]
create_dir = "./"

[read]
# Highlight
# Supported Languages => Try it out
# https://prismjs.com/index.html#supported-languages
# "Language to be loaded" (For Highlights)
read_lang = ""
# "Extension to be loaded"
# `ext` => OK
# `.ext` => NG
read_ext = ""
# Comment
# Supported symbol => `/`, `*`, `#`, `"`,
cmt_start = "/**"
# Target syntax
# ["function","class",...]
target_list = [""]

[exclude]
# ["Name of the file to exclude", ...]
# Pattern match from the top
exclude_filename = [""]
"#;

pub const TOML_PATH: &str = "./setting.toml";

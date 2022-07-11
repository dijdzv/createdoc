pub const TOML: &str = r#"[dir]
read_dir = "./"
read_folder = []
create_dir = "./"

[read]
# Highlight
# Supported Languages => php,js,rust,go
# https://prismjs.com/index.html#supported-languages
# "読み込みたい言語" (highlight)
read_lang = ""
# "読み込みたい拡張子"
read_filename_extension = ""
# Comment
# Supported symbol => /, *, #, ",
cmt_start = "/**"
# Target syntax
# ["function","class","fn","pub fn",...]
target = [""]

[exclude]
# ["除外したいファイル名", ...]
# Pattern match from the top
ex_filename = [""]"#;

pub const TOML_PATH: &str = "./setting.toml";

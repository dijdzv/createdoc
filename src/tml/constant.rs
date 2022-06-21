pub const TOML: &str = r#"[dir]
read_dir = "./"
create_dir = "./"

[read]
# 対応言語 php,js,rust,go
# "読み込みたい言語"
# https://prismjs.com/index.html#supported-languages
read_lang = ""
# "読み込みたい拡張子"
read_filename_extension = ""
# コメント
cmt_start = "/**"
# 対象
# ["function","class",...]
target = [""]

[exclude]
# ["除外したいファイル名", ...]
# 先頭からパターンマッチ
ex_filename = [""]"#;

pub const TOML: &str = r#"[dir]
read_dir = "./"
create_dir = "./"

[read]
# 対応言語 rust,php,js
# "読み込みたい言語"
# https://prismjs.com/index.html#supported-languages
read_lang = "rust"
# "読み込みたい拡張子"
# 後方からパターンマッチ
read_filename_extension = "rs"
# コメント
cmt_start = "/**"
cmt_end = " */"
# 対象
target = ["fn"]

[exclude]
# ["除外したいファイル名", ...]
# 先頭からパターンマッチ
ex_filename = [""]"#;

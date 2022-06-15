pub const TOML: &str = r#"[file]
# html
create_filename = "rsdoc"

[dir]
read_dir = "./"
create_dir = "./"

[read]
# https://prismjs.com/index.html#supported-languages
# "読み込みたい言語"
read_lang = "rust"
# "読み込みたい拡張子"
read_filename_extension = "rs"
# コメント
cmt_start = "/**"
cmt_end = " */"
# 対象
target = ["fn"]

[exclude]
# ["除外したいファイル名", ...]
# ファイル名の先頭から指定
ex_filename = [""]"#;

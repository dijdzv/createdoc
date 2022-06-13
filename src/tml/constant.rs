pub const TOML: &str = r#"[file]
# html
create_filename = "rsdoc"

[dir]
read_dir = "./"
create_dir = "./"

[read]
# "読み込みたい拡張子"
read_filename_extension = "php"
cmt_start = "/**"
cmt_end = " */"

[exclude]
# ["除外したいファイル名", ...]
# ファイル名の先頭から指定
ex_filename = ["index.php"]"#;

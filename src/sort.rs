pub fn sorting(
    filenames: Vec<String>,
    read_filename_extension: &str,
    ex_filename: Vec<String>,
) -> Vec<String> {
    // 拡張子で指定
    let filenames = filenames
        .into_iter()
        .filter(|f| f.ends_with(read_filename_extension));

    // 除外するファイル名を指定
    filenames
        .filter(|f| ex_filename.iter().all(|ef| !f.starts_with(ef)))
        .collect::<Vec<_>>()
}

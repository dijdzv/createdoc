pub fn sorting(
    filenames: Vec<String>,
    read_filename_extension: &str,
    ex_filename: Vec<String>,
) -> Vec<String> {
    let filenames = filenames
        .into_iter()
        .filter(|f| f.ends_with(read_filename_extension))
        .collect::<Vec<_>>();

    filenames
}

use crate::FolderVec;

pub fn search_data(folder_vec: &FolderVec) -> String {
    let filenames = folder_vec
        .iter()
        .map(|(filename, _)| filename.as_str())
        .collect::<Vec<_>>();

    filenames.join(" ")
}

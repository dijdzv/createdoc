use crate::FolderVec;

pub fn sort(folder_vec: &mut FolderVec) -> FolderVec {
    let mut sorted_file = folder_vec
        .iter_mut()
        .map(|(f, file_vec)| {
            file_vec.sort_by(|a, b| a.0.cmp(&b.0));
            (f.to_owned(), file_vec.clone())
        })
        .collect::<Vec<_>>();

    sorted_file.sort_by(|a, b| a.0.cmp(&b.0));

    sorted_file
}

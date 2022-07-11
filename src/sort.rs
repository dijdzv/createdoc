use crate::FolderVec;

pub fn sort(folder_vec: &mut FolderVec) -> FolderVec {
    folder_vec
        .iter_mut()
        .map(|(f, file_vec)| {
            file_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            (f.to_owned(), file_vec.clone())
        })
        .collect::<Vec<_>>()
}

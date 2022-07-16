use crate::DirVec;

pub fn sort(dir_vec: &mut DirVec) -> DirVec {
    let mut sorted_file = dir_vec
        .iter_mut()
        .map(|(f, file_vec)| {
            file_vec.sort_by(|a, b| a.0.cmp(&b.0));
            (f.to_owned(), file_vec.clone())
        })
        .collect::<Vec<_>>();

    sorted_file.sort_by(|a, b| a.0.cmp(&b.0));

    sorted_file
}

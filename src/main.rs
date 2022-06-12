use std::fs::File;
use std::io::{BufRead, BufReader};

mod add;
mod create;
mod read;
mod sort;

fn main() {
    let (mut create_filename, read_dir, create_dir, read_filename_extension, ex_filename) =
        read::read_toml();

    let mut buf = Vec::new(); // 一時保管
    let mut pair = Vec::new(); //docとfuncのペア
    let mut file_vec = Vec::new(); // １つのfile
    let mut folder_vec = Vec::new(); // 全てのファイル
    let mut func_name = String::from(""); // 関数名のbuf
    let mut pre = 0; // 前の行番号
    let mut is_doc = false;
    let mut is_fn = false;

    let mut filenames = read::read_dir(read_dir).unwrap();
    filenames = sort::sorting(filenames, &read_filename_extension, ex_filename);

    // folderに格納
    for filename in &filenames {
        // fileに格納
        for (i, result) in BufReader::new(File::open(filename).unwrap())
            .lines()
            .enumerate()
        {
            let l = result.ok().unwrap();

            // vecに行毎追加
            add::add_line(
                &l,
                (i, &mut pre),
                (&mut is_doc, &mut is_fn),
                (&mut buf, &mut func_name),
                &mut pair,
                &mut file_vec,
            );
        }
        folder_vec.push((filename, file_vec.clone()));
        file_vec.clear();
    }

    create::create_html(&create_dir, &mut create_filename, &folder_vec);
}

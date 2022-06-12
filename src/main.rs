use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod add;
mod create;
mod read;

fn main() {
    let (
        create_filename,
        read_dir,
        create_dir,
        read_filename,
        read_filename_extension,
        ex_filename,
    ) = read::read_toml();
    let filename = Path::new("./csv.php");

    let mut buf = Vec::new(); // 一時保管
    let mut pair = Vec::new(); //docとfuncのペア
    let mut file_vec = Vec::new(); // file毎のvec
    let mut func_name = String::from(""); // 関数名のbuf
    let mut pre = 0; // 前の行番号
    let mut is_doc = false;
    let mut is_fn = false;

    let filenames = read::read_dir(read_dir).unwrap();
    dbg!(filenames);

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

    create::create_html(&create_dir, &create_filename, &file_vec);
}

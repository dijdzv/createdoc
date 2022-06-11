use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod add;
mod create;

fn main() {
    let path = Path::new("./csv.php");

    let mut buf = Vec::new(); // 一時保管
    let mut pair = Vec::new(); //docとfuncのペア
    let mut file_vec = Vec::new(); // file毎のvec
    let mut func_name = String::from(""); // 関数名のbuf
    let mut pre = 0; // 前の行番号
    let mut is_doc = false;
    let mut is_fn = false;

    // 行ごとに格納
    for (i, result) in BufReader::new(File::open(path).unwrap())
        .lines()
        .enumerate()
    {
        let l = result.ok().unwrap();

        // vecに中身を追加
        add::add_line(
            &l,
            (i, &mut pre),
            (&mut is_doc, &mut is_fn),
            (&mut buf, &mut func_name),
            &mut pair,
            &mut file_vec,
        );
    }

    create::create_html(&file_vec);
}

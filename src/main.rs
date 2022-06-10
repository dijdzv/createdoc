use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod add;
mod create;

fn main() {
    let path = Path::new("./file.php");

    let mut buf = Vec::new();
    let mut pair = Vec::new();
    let mut file_vec = Vec::new();
    let mut func_name = String::from("");
    let mut pre = 0;
    let mut is_doc = false;
    let mut is_fn = false;

    // ファイルごとに格納
    for (i, result) in BufReader::new(File::open(path).unwrap())
        .lines()
        .enumerate()
    {
        let l = result.ok().unwrap();

        add::add_buf(
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

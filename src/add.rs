/// 関数とDocのvecを生成
pub fn add_buf(
    l: &str,
    (i, pre): (usize, &mut usize),
    (is_doc, is_fn): (&mut bool, &mut bool),
    (buf, func_name): (&mut Vec<String>, &mut String),
    pair: &mut Vec<Vec<String>>,
    file_vec: &mut Vec<(String, Vec<String>, Vec<String>)>,
) {
    if l.starts_with("/**") {
        buf.push(l.to_string());
        *is_doc = true; // doc start
    } else if l.starts_with(" */") && *is_doc {
        buf.push(l.to_string());
        *is_doc = false; // doc end
        pair.push(buf.to_vec());
        buf.clear();
        *pre = i;
    } else if l.starts_with("function") {
        // docとfnが隣あっていなければdocを空にしてpush
        if *pre != i - 1 {
            pair.clear();
            pair.push(["".to_string()].to_vec());
        }
        buf.push(l.to_string());
        *is_fn = true; // fn start
        let s = l.find(' ').unwrap();
        let e = l.find('(').unwrap();
        *func_name = l[s + 1..e].to_string(); // function名を取得
    } else if l.starts_with('}') && *is_fn {
        buf.push(l.to_string());
        *is_fn = false; // fn end
        pair.push(buf.to_vec()); //無駄
        buf.clear();
        file_vec.push((func_name.to_string(), pair[0].clone(), pair[1].to_owned()));
        pair.clear();
    } else if *is_doc || *is_fn {
        buf.push(l.to_string());
    }
}

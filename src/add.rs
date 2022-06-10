/// 関数とDocのvecを生成
pub fn add_buf(
    l: &str,
    (i, pre): (usize, &mut usize),
    (is_doc, is_fn): (&mut bool, &mut bool),
    buf: &mut Vec<String>,
    pair: &mut Vec<Vec<String>>,
    file_vec: &mut Vec<Vec<Vec<String>>>,
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
        // docとfnが隣あっていれば
        if *pre != i - 1 {
            pair.clear();
            pair.push(["".to_string()].to_vec());
        }
        buf.push(l.to_string());
        *is_fn = true; // fn start
    } else if l.starts_with('}') && *is_fn {
        buf.push(l.to_string());
        *is_fn = false; // fn end
        pair.push(buf.to_vec());
        buf.clear();
        file_vec.push([pair[0].to_vec(), pair[1].to_vec()].to_vec());
        pair.clear();
    } else if *is_doc || *is_fn {
        buf.push(l.to_string());
    }
}

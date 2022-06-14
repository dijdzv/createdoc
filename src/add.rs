type Buf = Vec<String>;
type Pair = Vec<Buf>;

/// 関数とDocのvecを生成
pub fn add_line(
    l: &str,
    (i, pre): (usize, &mut usize),
    (is_doc, is_fn): (&mut bool, &mut bool),
    (buf, func_name, pair, file_vec): (
        &mut Buf,
        &mut String,
        &mut Pair,
        &mut Vec<(String, Buf, Buf)>,
    ),
    (cmt_start, cmt_end): (&str, &str),
    target: &[String],
) {
    if l.starts_with(cmt_start) {
        buf.push(l.to_string());
        *is_doc = true; // doc start
        return;
    } else if l.starts_with(cmt_end) && *is_doc {
        buf.push(l.to_string());
        *is_doc = false; // doc end
        pair.push(buf.to_vec());
        buf.clear();
        *pre = i;
        return;
    }

    for t in target {
        if l.starts_with(t) {
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
            return;
        }
    }
    if l.starts_with('}') && *is_fn {
        buf.push(l.to_string());
        *is_fn = false; // fn end
        pair.push(buf.to_vec()); //無駄
        buf.clear();
        file_vec.push((func_name.to_string(), pair[0].clone(), pair[1].to_owned()));
        pair.clear();
    }
    if *is_doc || *is_fn {
        buf.push(l.to_string());
    }
}

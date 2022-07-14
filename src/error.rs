pub enum ErrorMsg {
    Captures,
    FileName,
    FileStem,
    Get,
    Parent,
    ToStr,
}

impl ErrorMsg {
    pub fn as_str(&self) -> &'static str {
        use ErrorMsg::*;
        match *self {
            Captures => "Contains invalid characters. [character,digit,_]",
            FileName => "The loaded path is terminated with `..`.",
            FileStem => "There is no file name.",
            Get => "Failed to obtain target name.",
            Parent => "The loaded Path ends with root or prefix.",
            ToStr => "The loaded file name is not in valid Unicode.",
        }
    }
}

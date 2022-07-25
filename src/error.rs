pub enum ErrorMsg {
    Captures,
    FileStem,
    Get,
    Parent,
}

impl ErrorMsg {
    pub fn as_str(&self) -> &'static str {
        use ErrorMsg::*;
        match *self {
            Captures => "Contains invalid characters. [ASCII[a-zA-Z],digit,Unicode[(unknown)]]",
            FileStem => "There is no file name.",
            Get => "Failed to obtain target name.",
            Parent => "The loaded Path ends with root or prefix.",
        }
    }
}

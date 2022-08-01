use std::{
    fmt::Display as D,
    fs::File,
    io::{self, Write},
};
/// ファイル出力用のstruct
pub struct Output {
    code: String,
}

impl Output {
    pub fn add<T: D>(&mut self, code: T) {
        *self = Self {
            code: format!("{}{}", self.code, code),
        };
    }
    pub fn new() -> Self {
        Output {
            code: String::new(),
        }
    }
    pub fn write(&self, file: &mut File) -> io::Result<()> {
        file.write_all(self.code.as_bytes())
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

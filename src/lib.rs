use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
};

pub struct Output {
    pub code: String,
}

impl Output {
    pub fn new() -> Self {
        Output {
            code: String::new(),
        }
    }
    pub fn add<T: Display>(&mut self, code: T) {
        *self = Self {
            code: format!("{}{}", self.code, code),
        };
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

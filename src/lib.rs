use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
};

pub struct Output {
    pub output_string: String,
}

impl Output {
    pub fn new() -> Self {
        Output {
            output_string: "".to_string(),
        }
    }
    pub fn add<T: Display>(&mut self, code: T) {
        *self = Self {
            output_string: format!("{}{}", self.output_string, code),
        };
    }
    pub fn write(&self, file: &mut File) -> io::Result<()> {
        file.write_all(self.output_string.as_bytes())
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

use std::{
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
    pub fn add(&mut self, source: &str) {
        *self = Self {
            output_string: format!("{}{}", self.output_string, source),
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

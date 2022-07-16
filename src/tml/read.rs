use crate::tml;
use createdoc::Setting;
use std::{
    fs::read_to_string,
    io::{self, ErrorKind},
    path::Path,
};

pub fn read_toml() -> Result<Setting, io::Error> {
    let result = read_to_string(tml::TOML_PATH);
    let s = match result {
        Ok(s) => s,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            tml::create_toml(tml::TOML, Path::new(tml::TOML_PATH))?;
            let new_e = io::Error::new(
                ErrorKind::NotFound,
                "The `setting.toml` file did not exist, so I created a new one.",
            );
            return Err(new_e);
        }
        Err(e) => return Err(e),
    };

    let setting = toml::from_str(&s)?;

    Ok(setting)
}

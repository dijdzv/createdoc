use crate::tml;
use crate::Setting;
use anyhow::bail;
use std::{fs::read_to_string, io::ErrorKind, path::Path};

pub fn read_toml() -> anyhow::Result<Setting> {
    let result = read_to_string(tml::TOML_PATH);
    let s = match result {
        Ok(s) => s,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            tml::create_toml(tml::TOML, Path::new(tml::TOML_PATH))?;
            bail!("The `setting.toml` file did not exist, so I created a new one.");
        }
        Err(e) => bail!(e),
    };

    let setting = toml::from_str(&s)?;

    Ok(setting)
}

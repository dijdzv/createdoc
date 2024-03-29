use crate::ReadData;
use crate::{add, create, read, tml};

pub fn run() -> anyhow::Result<String> {
    let setting = tml::read_toml()?;
    let filepaths = read::read_control(&setting)?;
    let mut read_data = ReadData::new(&setting);

    add::add_control(filepaths, &mut read_data)?;

    create::create_html(
        &setting.create_filepath(),
        setting.read_lang(),
        &read_data.file_and_syntax_categorize(),
    )?;

    Ok(setting.create_filepath())
}

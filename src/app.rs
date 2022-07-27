use crate::{add, create, read, tml};
use createdoc::ReadData;

pub fn run() -> anyhow::Result<String> {
    let setting = tml::read_toml()?;
    let filepaths = read::read_control(&setting)?;
    let mut read_data = ReadData::new(&setting);

    add::add_control(filepaths, &mut read_data)?;

    create::create_html(
        &setting.create_filepath(),
        setting.read_lang(),
        &read_data.syntax_categorize(),
    )?;

    Ok(setting.create_filepath())
}

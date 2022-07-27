mod add;
mod create;
mod error;
mod read;
mod tml;

use createdoc::ReadData;

fn main() {
    match app() {
        Ok(create_filepath) => {
            println!("\x1b[32mCreated `{}` successfully!\x1b[m", create_filepath)
        }
        Err(e) => eprintln!("\x1b[31merror:\x1b[m {}", e),
    }
}

fn app() -> anyhow::Result<String> {
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

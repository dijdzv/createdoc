mod add;
mod create;
mod error;
mod read;
mod tml;

use createdoc::ReadData;

fn main() {
    match app() {
        Ok(create_filepath) => println!("Created `{}` successfully!", create_filepath),
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}

fn app() -> anyhow::Result<String> {
    let setting = tml::read_toml()?;
    let filepaths = read::read_control(&setting)?;

    let mut read_data = ReadData::new(&setting);

    add::add_control(filepaths, &mut read_data)?;

    let read_lang = setting.read_lang();
    create::create_html(setting.create_dir(), read_lang, &read_data.dir_vec)?;

    Ok(format!("{}doc.html", read_lang))
}

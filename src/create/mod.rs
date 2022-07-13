use super::FolderVec;
use std::{fs::File, io::Write};

mod constant;
mod main;
mod nav;
mod search;

/// htmlファイルを生成
pub fn create_html(
    create_dir: &str,
    read_lang: &str,
    folder_vec: &FolderVec,
) -> Result<(), Box<dyn std::error::Error>> {
    let create_filepath = format!("{}{}doc", create_dir, read_lang);

    let mut file = File::create(create_filepath + ".html")?;
    // html
    file.write_all(constant::HTML_TOP_START.as_bytes())?;
    file.write_all(constant::STYLE.as_bytes())?;
    file.write_all(constant::PRISM_CDN_CSS.as_bytes())?;
    file.write_all(constant::HTML_TOP_END.as_bytes())?;

    // wrap
    file.write_all(r#"<div class="wrap">"#.as_bytes())?;

    nav::create_nav(&mut file, folder_vec, read_lang);

    main::create_main(&mut file, folder_vec, read_lang)?;

    // /wrap
    file.write_all("</div>".as_bytes())?;

    // script
    file.write_all(constant::SCRIPT.as_bytes())?;
    file.write_all(constant::PRISM_CDN_JS.as_bytes())?;
    file.write_all(constant::PRISM_AUTO_LOADER.as_bytes())?;

    file.write_all(search::SEARCH_SCRIPT.as_bytes())?;

    // /html
    file.write_all(constant::HTML_BOTTOM.as_bytes())?;

    Ok(())
}

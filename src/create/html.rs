use createdoc::Output;

use super::constant;
use super::main;
use super::nav;
use super::search;
use crate::FolderVec;

use std::{fs::File, io::Write, path::Path};

pub fn create_html<P: AsRef<Path>>(
    create_dir: P,
    read_lang: &str,
    folder_vec: &FolderVec,
) -> Result<(), Box<dyn std::error::Error>> {
    let create_filepath = format!("{}doc.html", create_dir.as_ref().join(read_lang).display());

    let mut file = File::create(create_filepath)?;
    let mut output = Output::new();
    // html
    file.write_all(constant::HTML_TOP_START.as_bytes())?;
    file.write_all(constant::STYLE.as_bytes())?;
    file.write_all(constant::PRISM_CDN_CSS.as_bytes())?;
    file.write_all(constant::HTML_TOP_END.as_bytes())?;

    // wrap
    file.write_all(br#"<div class="wrap">"#)?;

    nav::create_nav(&mut file, folder_vec, read_lang)?;

    main::create_main(&mut file, folder_vec, read_lang)?;

    // /wrap
    file.write_all(b"</div>")?;

    // script
    file.write_all(constant::SCRIPT.as_bytes())?;
    file.write_all(constant::PRISM_CDN_JS.as_bytes())?;
    file.write_all(constant::PRISM_AUTO_LOADER.as_bytes())?;

    file.write_all(search::SEARCH_SCRIPT.as_bytes())?;

    // /html
    file.write_all(constant::HTML_BOTTOM.as_bytes())?;

    Ok(())
}

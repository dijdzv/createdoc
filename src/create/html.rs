use createdoc::Output;

use super::constant;
use super::main;
use super::nav;
use super::search;
use crate::FolderVec;

use std::{fs::File, path::Path};

pub fn create_html<P: AsRef<Path>>(
    create_dir: P,
    read_lang: &str,
    folder_vec: &FolderVec,
) -> Result<(), Box<dyn std::error::Error>> {
    let create_filepath = format!("{}doc.html", create_dir.as_ref().join(read_lang).display());

    let mut file = File::create(create_filepath)?;
    let mut output = Output::new();
    // html
    output.add(constant::HTML_TOP_START);
    output.add(constant::STYLE);
    output.add(constant::PRISM_CDN_CSS);
    output.add(constant::HTML_TOP_END);

    // wrap
    output.add(r#"<div class="wrap">"#);

    nav::create_nav(&mut output, folder_vec, read_lang)?;

    main::create_main(&mut output, folder_vec, read_lang)?;

    // /wrap
    output.add("</div>");

    // script
    output.add(constant::SCRIPT);
    output.add(constant::PRISM_CDN_JS);
    output.add(constant::PRISM_AUTO_LOADER);

    output.add(search::SEARCH_SCRIPT);

    // /html
    output.add(constant::HTML_BOTTOM);

    output.write(&mut file)?;

    Ok(())
}

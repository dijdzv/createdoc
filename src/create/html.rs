use super::constant;
use super::main;
use super::nav;
use super::search;
use createdoc::Categorized;
use createdoc::{AllVec, Output};

use std::fs::File;

pub fn create_html(
    create_filepath: &str,
    read_lang: &str,
    all: &AllVec,
    categorized: &Categorized,
) -> anyhow::Result<()> {
    let mut file = File::create(create_filepath)?;
    let mut output = Output::new();
    // html
    output.add(constant::HTML_TOP_START);
    output.add(constant::STYLE);
    output.add(constant::PRISM_CDN_CSS);
    output.add(constant::HTML_TOP_END);

    // wrap
    output.add(r#"<div class="wrap">"#);

    nav::create_nav(&mut output, categorized, read_lang)?;

    main::create_main(&mut output, all, categorized, read_lang)?;

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

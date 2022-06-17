use headless_chrome::{protocol::target::methods::CreateTarget, Browser};
use std::error::Error;

pub fn browse_open() -> std::result::Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());

    Ok(())
}

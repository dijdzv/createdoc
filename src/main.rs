use clap::App;
use createdoc::app;

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    match app::run() {
        Ok(create_filepath) => {
            println!(
                "\x1b[32mCreated \x1b[34m`{}`\x1b[32m successfully!\x1b[m",
                create_filepath
            )
        }
        Err(e) => eprintln!("\x1b[31merror:\x1b[m {}", e),
    }
}

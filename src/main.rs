use std::process::exit;

pub mod blocks;
pub mod country_code;
pub mod game;
pub mod game_version;
pub mod hash;
pub mod save;
pub mod stream;
#[cfg(feature = "gui")]
pub mod ui;

fn start() -> Result<(), Box<dyn std::error::Error>> {
    Ok(ui::app::run()?)
}

#[cfg(feature = "gui")]
fn main() {
    if let Err(e) = start() {
        eprint!("{e}");
        exit(1);
    }
}

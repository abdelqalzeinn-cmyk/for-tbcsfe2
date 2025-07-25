pub mod blocks;
pub mod country_code;
pub mod game;
pub mod game_version;
pub mod hash;
#[cfg(feature = "network")]
pub mod network;
pub mod save;
pub mod stream;
#[cfg(feature = "gui")]
pub mod ui;

#[cfg(feature = "gui")]
fn start() -> Result<(), Box<dyn std::error::Error>> {
    Ok(ui::app::run()?)
}

#[cfg(feature = "gui")]
fn main() {
    if let Err(e) = start() {
        eprint!("{e}");
        std::process::exit(1);
    }
}
#[cfg(not(feature = "gui"))]
fn main() {}

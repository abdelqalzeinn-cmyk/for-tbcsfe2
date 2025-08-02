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
fn main() {
    let mut args = std::env::args();
    let filepath = args.nth(1);
    if let Err(e) = ui::app::run(filepath.map(std::path::PathBuf::from)) {
        eprint!("{e}");
        std::process::exit(1);
    }
}
#[cfg(not(feature = "gui"))]
fn main() {}

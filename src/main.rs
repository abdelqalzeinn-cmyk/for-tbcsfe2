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

#[cfg(feature = "wasm")]
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    if let Err(e) = ui::app::run_wasm() {
        eprint!("{e}");
        std::process::exit(1);
    }
}

#[cfg(feature = "gui")]
#[cfg(not(feature = "wasm"))]
fn main() {
    let mut args = std::env::args();
    let filepath = args.nth(1);
    let asset_path = args.next();

    if let Err(e) = ui::app::run(
        filepath.map(std::path::PathBuf::from),
        asset_path.map(std::path::PathBuf::from).as_deref(),
    ) {
        eprint!("{e}");
        std::process::exit(1);
    }
}

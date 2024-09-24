#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bunyi::app::App;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let _ = App::run("Bunyi");
}

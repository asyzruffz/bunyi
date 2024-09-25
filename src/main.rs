#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bunyi::app::run_app;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let _ = run_app("Bunyi");
}

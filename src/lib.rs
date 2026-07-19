mod commands;
mod error;
mod notify;
mod nvim;

use nvim_oxi::{self as oxi};
use oxi::Dictionary;

#[oxi::plugin]
fn patchwise() -> Dictionary {
    notify::info("Patchwise loaded");

    if let Err(error) = commands::register_all() {
        notify::error(&format!("Patchwise initialization failed: {error}"));
    };

    Dictionary::new()
}

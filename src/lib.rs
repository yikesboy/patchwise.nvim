mod background;
mod commands;
mod error;
mod feature;
mod nvim;
mod prompt;
mod provider;

use nvim::notify;
use nvim_oxi::{self as oxi};
use oxi::Dictionary;

use crate::error::Result;

#[oxi::plugin]
fn patchwise() -> Dictionary {
    match initialize() {
        Ok(()) => notify::info("Patchwise loaded"),
        Err(error) => notify::error(&format!("Patchwise initialization failed: {error}")),
    }

    Dictionary::new()
}

fn initialize() -> Result<()> {
    background::init()?;
    nvim::dispatch::init()?;
    commands::register_all()
}

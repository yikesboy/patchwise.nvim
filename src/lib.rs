use api::types::LogLevel;
use nvim_oxi as oxi;
use oxi::{Dictionary, api};

const PLUGIN_LOGLEVEL: LogLevel = LogLevel::Info;

#[oxi::plugin]
fn patchwise() -> Dictionary {
    api::notify("Patchwise loaded", PLUGIN_LOGLEVEL, &Default::default())
        .expect("failed to send notification");

    Dictionary::new()
}

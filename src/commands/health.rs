use std::process::{Command, Stdio};

use nvim_oxi::api::types::CommandArgs;

use crate::error::{PatchwiseError, Result};
use crate::notify;

pub fn run(_args: CommandArgs) -> Result<()> {
    let status = Command::new("codex")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .map_err(PatchwiseError::CodexIO)?;

    if !status.success() {
        return Err(PatchwiseError::CodexFailed(status));
    }

    notify::info("Patchwise is healthy!");

    Ok(())
}

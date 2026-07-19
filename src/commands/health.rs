use nvim_oxi::api::types::CommandArgs;

use crate::error::Result;
use crate::nvim::notify;
use crate::provider;

pub fn run(_args: CommandArgs) -> Result<()> {
    provider::heatlh()?;
    notify::info("Patchwise is installed and authenticated!");
    Ok(())
}

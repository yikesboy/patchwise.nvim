use nvim_oxi::api::types::CommandArgs;

use crate::error::{PatchwiseError, Result};
use crate::feature::edit;

pub fn run(args: CommandArgs) -> Result<()> {
    let instruction = args
        .args
        .as_deref()
        .map(str::trim)
        .filter(|instruction| !instruction.is_empty())
        .ok_or(PatchwiseError::MissingInstruction)?;

    edit::start(instruction)
}

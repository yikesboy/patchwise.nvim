use nvim_oxi::api::types::CommandArgs;

use crate::error::Result;
use crate::notify;
use crate::nvim::buffer::PatchwiseBuffer;
use crate::nvim::selection::Selection;

pub fn run(_args: CommandArgs) -> Result<()> {
    let buffer = PatchwiseBuffer::current();
    let selection = Selection::current(&buffer)?;

    notify::info(&selection.text);

    Ok(())
}

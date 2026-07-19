use crate::error::Result;
use crate::nvim::buffer::PatchwiseBuffer;
use crate::nvim::selection::Selection;

use nvim_oxi::api::types::CommandArgs;

const MOCK_REPLACEMENT: &str = "TEST\nTEST\nTEST";

pub fn run(_args: CommandArgs) -> Result<()> {
    let mut buffer = PatchwiseBuffer::current();
    let selection = Selection::current(&buffer)?;
    buffer.replace(selection.range, MOCK_REPLACEMENT)
}

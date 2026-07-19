use crate::{prompt, provider};
use nvim_oxi::api::types::CommandArgs;

use crate::{
    edit::EditRequest,
    error::{PatchwiseError, Result},
    nvim::{buffer::PatchwiseBuffer, notify, selection::Selection},
};

pub fn run(args: CommandArgs) -> Result<()> {
    let instruction = args
        .args
        .as_deref()
        .map(str::trim)
        .filter(|instruction| !instruction.is_empty())
        .ok_or(PatchwiseError::MissingInstruction)?;

    let mut buffer = PatchwiseBuffer::current();
    let selection = Selection::current(&buffer)?;

    let request = EditRequest {
        instruction: instruction.to_owned(),
        selection: selection.text,
        context: buffer.text()?,
        file_path: buffer.file_path()?,
        file_type: buffer.file_type()?,
    };

    let prompt = prompt::edit::build_edit_prompt(&request);

    notify::info("Patchwise: generating replacement");

    let replacement = provider::generate(&prompt)?;
    buffer.replace(selection.range, &replacement)?;

    notify::info("Patchwise: replacement applied");

    Ok(())
}

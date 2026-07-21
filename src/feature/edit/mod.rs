mod pending_edit;
use std::path::PathBuf;

use crate::{
    background,
    error::Result,
    feature::edit::pending_edit::PendingEdit,
    nvim::{buffer::PatchwiseBuffer, notify, selection::Selection},
    prompt, provider,
};

type GenerationResult = std::result::Result<String, String>;

#[derive(Debug)]
pub struct EditRequest {
    pub instruction: String,
    pub selection: String,
    pub context: String,
    pub file_path: PathBuf,
    pub file_type: String,
}

pub fn start(instruction: &str) -> Result<()> {
    let mut buffer = PatchwiseBuffer::current();
    let selection = Selection::current(&buffer)?;

    let request = EditRequest {
        instruction: instruction.to_owned(),
        selection: selection.text.clone(),
        context: buffer.text()?,
        file_path: buffer.file_path()?,
        file_type: buffer.file_type()?,
    };

    let prompt = prompt::edit::build(&request);
    let pending_edit = PendingEdit::create(&mut buffer, &selection)?;

    background::run(
        move || provider::generate(&prompt).map_err(|error| error.to_string()),
        move |result| {
            finish(pending_edit, result);
        },
    )?;

    notify::info("Patchwise: generating replacement");

    Ok(())
}

fn finish(pending_edit: PendingEdit, generation: GenerationResult) {
    let result = pending_edit.complete(generation);
    match result {
        Ok(()) => notify::info("Patchwise: edit applied"),
        Err(error) => notify::error(&format!("PatchwiseEdit: {error}")),
    }
}

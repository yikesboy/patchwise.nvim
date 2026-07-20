use std::path::PathBuf;

use crate::error::Result;
use crate::prompt;

use crate::{
    background,
    error::PatchwiseError,
    nvim::{buffer::PatchwiseBuffer, notify, selection::Selection},
    provider,
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

#[derive(Debug)]
struct PendingEdit {
    buffer_handle: i32,
    selection: Selection,
}

pub fn start(instruction: &str) -> Result<()> {
    let buffer = PatchwiseBuffer::current();
    let selection = Selection::current(&buffer)?;

    let request = EditRequest {
        instruction: instruction.to_owned(),
        selection: selection.text.clone(),
        context: buffer.text()?,
        file_path: buffer.file_path()?,
        file_type: buffer.file_type()?,
    };

    let prompt = prompt::edit::build(&request);

    let pending_edit = PendingEdit {
        buffer_handle: buffer.handle(),
        selection,
    };

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
    let result = generation
        .map_err(PatchwiseError::BackgroundProvider)
        .and_then(|replacement| pending_edit.apply(&replacement));

    match result {
        Ok(()) => notify::info("Patchwise: edit applied"),
        Err(error) => notify::error(&format!("PatchwiseEdit: {error}")),
    }
}

impl PendingEdit {
    fn apply(self, replacement: &str) -> Result<()> {
        let mut buffer = PatchwiseBuffer::from_handle(self.buffer_handle);

        if !buffer.is_valid() {
            return Err(PatchwiseError::BufferUnavailable {
                buffer: self.buffer_handle,
            });
        }

        let current_selection = buffer.read(self.selection.range)?;

        if current_selection != self.selection.text {
            return Err(PatchwiseError::SelectionChanged);
        }

        buffer.replace(self.selection.range, replacement)
    }
}

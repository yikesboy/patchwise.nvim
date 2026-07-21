use std::sync::OnceLock;

use crate::error::{PatchwiseError, Result};
use crate::feature::edit::GenerationResult;
use crate::nvim::selection::TextRange;
use crate::nvim::{buffer::PatchwiseBuffer, selection::Selection};

const NAMESPACE_NAME: &str = "patchwise.edit.tracking";
const START_RIGHT_GRAVITY: bool = true;
const END_RIGHT_GRAVITY: bool = true;

static NAMESPACE: OnceLock<u32> = OnceLock::new();

pub struct PendingEdit {
    buffer_handle: i32,
    namespace: u32,
    start_extmark: u32,
    end_extmark: u32,
    original_text: String,
}

impl PendingEdit {
    pub fn create(buffer: &mut PatchwiseBuffer, selection: &Selection) -> Result<Self> {
        let namespace = namespace();

        let start_extmark =
            buffer.create_extmark(namespace, selection.range.start, START_RIGHT_GRAVITY)?;

        let end_extmark_result =
            buffer.create_extmark(namespace, selection.range.end, END_RIGHT_GRAVITY);

        let end_extmark = match end_extmark_result {
            Ok(extmark) => extmark,
            Err(error) => {
                let _ = buffer.delete_extmark(namespace, start_extmark);
                return Err(error);
            }
        };

        Ok(Self {
            buffer_handle: buffer.handle(),
            namespace,
            start_extmark,
            end_extmark,
            original_text: selection.text.clone(),
        })
    }

    pub fn complete(self, generation: GenerationResult) -> Result<()> {
        let operation = generation
            .map_err(PatchwiseError::BackgroundProvider)
            .and_then(|replacement| self.apply(&replacement));
        let cleanup = self.clear();

        operation.and(cleanup)
    }

    fn apply(&self, replacement: &str) -> Result<()> {
        let mut buffer = PatchwiseBuffer::from_handle(self.buffer_handle);
        if !buffer.is_valid() {
            return Err(PatchwiseError::BufferUnavailable {
                buffer: self.buffer_handle,
            });
        }
        let range = self.resolve(&buffer)?;
        let current_text = buffer.read(range)?;

        if current_text != self.original_text {
            return Err(PatchwiseError::SelectionChanged);
        }

        buffer.replace(range, replacement)
    }

    fn resolve(&self, buffer: &PatchwiseBuffer) -> Result<TextRange> {
        let start = buffer.extmark_position(self.namespace, self.start_extmark)?;
        let end = buffer.extmark_position(self.namespace, self.end_extmark)?;
        Ok(TextRange { start, end })
    }

    fn clear(&self) -> Result<()> {
        let mut buffer = PatchwiseBuffer::from_handle(self.buffer_handle);
        if !buffer.is_valid() {
            return Ok(());
        }

        let start_result = buffer.delete_extmark(self.namespace, self.start_extmark);
        let end_result = buffer.delete_extmark(self.namespace, self.end_extmark);

        start_result.and(end_result)
    }
}

fn namespace() -> u32 {
    *NAMESPACE.get_or_init(|| nvim_oxi::api::create_namespace(NAMESPACE_NAME))
}

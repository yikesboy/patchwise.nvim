use std::sync::OnceLock;

use crate::error::{PatchwiseError, Result};
use crate::feature::edit::GenerationResult;
use crate::nvim::selection::TextRange;
use crate::nvim::{buffer::PatchwiseBuffer, selection::Selection};

const NAMESPACE_NAME: &str = "patchwise.edit.tracking";
const START_RIGHT_GRAVITY: bool = true;
const END_RIGHT_GRAVITY: bool = true;
const PENDING_SIGN_TEXT: &str = "|";
const PENDING_HIGHLIGHT: &str = "DiffChange";
const PENDING_SIGN_HIGHLIGHT: &str = "WarningMsg";

static NAMESPACE: OnceLock<u32> = OnceLock::new();

pub struct PendingEdit {
    buffer_handle: i32,
    namespace: u32,
    start_extmark: u32,
    end_extmark: u32,
    highlight_extmark: u32,
    sign_extmarks: Vec<u32>,
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

        let decoration_result = create_decorations(buffer, namespace, selection.range);
        let (highlight_extmark, sign_extmarks) = match decoration_result {
            Ok(decorations) => decorations,
            Err(error) => {
                let _ = buffer.delete_extmark(namespace, start_extmark);
                let _ = buffer.delete_extmark(namespace, end_extmark);
                return Err(error);
            }
        };

        Ok(Self {
            buffer_handle: buffer.handle(),
            namespace,
            start_extmark,
            end_extmark,
            highlight_extmark,
            sign_extmarks,
            original_text: selection.text.clone(),
        })
    }

    pub fn complete(self, generation: GenerationResult) -> Result<()> {
        let operation = generation
            .map_err(PatchwiseError::BackgroundProvider)
            .and_then(|replacement| {
                let replacement = strip_md_code_block(&replacement);
                self.apply(&replacement)
            });
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
        let mut result = start_result.and(end_result);

        result = result.and(buffer.delete_extmark(self.namespace, self.highlight_extmark));
        for sign_extmark in self.sign_extmarks.iter().copied() {
            result = result.and(buffer.delete_extmark(self.namespace, sign_extmark));
        }

        result
    }
}

fn create_decorations(
    buffer: &mut PatchwiseBuffer,
    namespace: u32,
    range: TextRange,
) -> Result<(u32, Vec<u32>)> {
    let highlight_extmark = buffer.create_highlighted_range(namespace, range, PENDING_HIGHLIGHT)?;

    let sign_extmarks_result = create_signs(buffer, namespace, range);
    let sign_extmarks = match sign_extmarks_result {
        Ok(sign_extmarks) => sign_extmarks,
        Err(error) => {
            let _ = buffer.delete_extmark(namespace, highlight_extmark);
            return Err(error);
        }
    };

    Ok((highlight_extmark, sign_extmarks))
}

fn create_signs(
    buffer: &mut PatchwiseBuffer,
    namespace: u32,
    range: TextRange,
) -> Result<Vec<u32>> {
    let mut signs = Vec::new();

    for row in range.start.row..=range.end.row {
        let sign_result =
            buffer.create_line_sign(namespace, row, PENDING_SIGN_TEXT, PENDING_SIGN_HIGHLIGHT);
        let sign = match sign_result {
            Ok(sign) => sign,
            Err(error) => {
                for sign in signs {
                    let _ = buffer.delete_extmark(namespace, sign);
                }
                return Err(error);
            }
        };
        signs.push(sign);
    }

    Ok(signs)
}

fn strip_md_code_block(response: &str) -> &str {
    if !response.starts_with("```") {
        return response;
    }

    let Some(body_start) = response.find('\n') else {
        return response;
    };

    let body = &response[body_start + 1..];

    body.strip_suffix("```")
        .map(str::trim_end)
        .unwrap_or(response)
}

fn namespace() -> u32 {
    *NAMESPACE.get_or_init(|| nvim_oxi::api::create_namespace(NAMESPACE_NAME))
}

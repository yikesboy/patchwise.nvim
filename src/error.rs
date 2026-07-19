use std::process::ExitStatus;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PatchwiseError {
    #[error("Codex exited with status {0}")]
    CodexFailed(ExitStatus),

    #[error("failed to run Codex")]
    CodexIO(#[source] std::io::Error),

    #[error("failed to register command '{name}")]
    CommandRegistration {
        name: &'static str,
        #[source]
        source: nvim_oxi::api::Error,
    },

    #[error("Failed to read selection from current buffer")]
    SelectionRead(#[source] nvim_oxi::api::Error),

    #[error("No selection in current buffer")]
    NoSelection,

    #[error("Failed to set text in buffer: {0}")]
    BufferEdit(#[source] nvim_oxi::api::Error),

    #[error("Invalid selection position at row {row}, column: {col}")]
    InvalidSelectionPosition { row: usize, col: usize },
}

pub type Result<T> = std::result::Result<T, PatchwiseError>;

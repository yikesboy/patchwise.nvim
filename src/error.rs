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
}

pub type Result<T> = std::result::Result<T, PatchwiseError>;

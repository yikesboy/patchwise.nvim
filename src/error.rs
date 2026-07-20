use std::process::ExitStatus;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PatchwiseError {
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

    #[error("'{provider}' is not installed or unavailable: {source}")]
    ProviderUnavailable {
        provider: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error("'{provider}' command '{executable} {argument}' exited with {status}")]
    ProviderCommandFailed {
        provider: &'static str,
        executable: &'static str,
        argument: &'static str,
        status: ExitStatus,
    },

    #[error("'{provider}' is not authenticated")]
    ProviderNotAuthenticated { provider: &'static str },

    #[error("Failed to start {provider}")]
    ProviderStart {
        provider: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed while waiting for {provider}: {source}")]
    ProviderWait {
        provider: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error("{provider} exited with {status}: {stderr}")]
    ProviderFailed {
        provider: &'static str,
        status: ExitStatus,
        stderr: String,
    },

    #[error("Failed to send the request to {provider}")]
    ProviderWrite {
        provider: &'static str,
        source: std::io::Error,
    },

    #[error("{provider} returned invalid UTF-8")]
    ProviderOutput {
        provider: &'static str,
        #[source]
        source: std::string::FromUtf8Error,
    },

    #[error("{provider} returned empty response")]
    EmptyProviderResponse { provider: &'static str },

    #[error("Failed to open stdin for {provider}")]
    ProviderStdin { provider: &'static str },

    #[error("{provider} stdin writer thread panicked")]
    ProviderStdinWriterPanicked { provider: &'static str },

    #[error("Missing instruction for commmand")]
    MissingInstruction,

    #[error("Unable to read current buffer: {0}")]
    BufferRead(#[source] nvim_oxi::api::Error),

    #[error("Failed to create runtime: {0}")]
    RuntimeInitialization(#[source] std::io::Error),

    #[error("Runtime is not initlialized")]
    RuntimeNotInitialized,

    #[error("Failed to initialize neovim main thread dispatcher: {0}")]
    DispatchInitiallization(#[source] nvim_oxi::libuv::Error),

    #[error("Neovim main thread dispatcher is not initlialized")]
    DispatchNotInitialized,

    #[error("Neovim main thread dispatcher is closed")]
    DispatchClosed,

    #[error("Failed to wake the neovim main thread: {0}")]
    DispatchWake(#[source] nvim_oxi::libuv::Error),

    #[error("Provider generation failed: {0}")]
    BackgroundProvider(String),

    #[error("Buffer {buffer} is no longer available")]
    BufferUnavailable { buffer: i32 },

    #[error("Selected text changed while generating. Edit not applied.")]
    SelectionChanged,
}

pub type Result<T> = std::result::Result<T, PatchwiseError>;

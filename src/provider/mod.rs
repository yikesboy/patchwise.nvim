mod codex;
mod process;

use std::path::PathBuf;

use crate::error::Result;
use crate::prompt::Prompt;
use crate::provider::codex::CodexProvider;

const CURRENT_PROVIDER: CodexProvider = CodexProvider;

pub struct ProviderRequest {
    pub prompt: Prompt,
    pub working_directory: Option<PathBuf>,
}

pub trait Provider {
    fn generate(&self, request: &ProviderRequest) -> Result<String>;
    fn health(&self) -> Result<()>;
}

pub fn generate(request: &ProviderRequest) -> Result<String> {
    CURRENT_PROVIDER.generate(request)
}

pub fn heatlh() -> Result<()> {
    CURRENT_PROVIDER.health()
}

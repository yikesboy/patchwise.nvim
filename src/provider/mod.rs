mod codex;
mod process;

use crate::error::Result;
use crate::prompt::Prompt;
use crate::provider::codex::CodexProvider;

const CURRENT_PROVIDER: CodexProvider = CodexProvider;

pub trait Provider {
    fn generate(&self, prompt: &Prompt) -> Result<String>;
    fn health(&self) -> Result<()>;
}

pub fn generate(prompt: &Prompt) -> Result<String> {
    CURRENT_PROVIDER.generate(prompt)
}

pub fn heatlh() -> Result<()> {
    CURRENT_PROVIDER.health()
}

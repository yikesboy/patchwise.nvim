use std::process::{Command, Stdio};

use crate::error::PatchwiseError;
use crate::error::Result;
use crate::prompt::Prompt;
use crate::provider::Provider;
use crate::provider::process::run_with_stdin;

const PROVIDER_NAME: &str = "Codex";
const EXECUTABLE: &str = "codex";
const VERSION_ARG: &str = "--version";
const LOGIN_ARG: &str = "login";
const STATUS_ARG: &str = "status";

pub struct CodexProvider;

impl Provider for CodexProvider {
    fn health(&self) -> Result<()> {
        ensure_available_on_path()?;
        ensure_is_authenticated()
    }

    fn generate(&self, prompt: &Prompt) -> Result<String> {
        ensure_is_authenticated()?;
        let command = build_command();
        let prompt_bytes = prompt.as_str().to_owned().into_bytes();
        let response = run_with_stdin(PROVIDER_NAME, command, prompt_bytes)?;
        normalize_response(&response)
    }
}

fn build_command() -> Command {
    let mut command = Command::new(EXECUTABLE);
    command.args([
        "exec",
        "--ephemeral",
        "--sandbox",
        "read-only",
        "--skip-git-repo-check",
        "-",
    ]);

    command
}

fn normalize_response(response: &str) -> Result<String> {
    let response = response.trim();

    if response.is_empty() {
        return Err(PatchwiseError::EmptyProviderResponse {
            provider: PROVIDER_NAME,
        });
    }

    let cleaned_response = strip_md_code_block(response).to_owned();

    Ok(cleaned_response)
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

fn ensure_available_on_path() -> Result<()> {
    let status = Command::new(EXECUTABLE)
        .arg(VERSION_ARG)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|source| PatchwiseError::ProviderUnavailable {
            provider: PROVIDER_NAME,
            source,
        })?;

    if !status.success() {
        return Err(PatchwiseError::ProviderCommandFailed {
            provider: PROVIDER_NAME,
            executable: EXECUTABLE,
            argument: VERSION_ARG,
            status,
        });
    }

    Ok(())
}

fn ensure_is_authenticated() -> Result<()> {
    let output = Command::new(EXECUTABLE)
        .args([LOGIN_ARG, STATUS_ARG])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|source| PatchwiseError::ProviderStart {
            provider: PROVIDER_NAME,
            source,
        })?;

    if !output.status.success() {
        return Err(PatchwiseError::ProviderNotAuthenticated {
            provider: PROVIDER_NAME,
        });
    }

    Ok(())
}

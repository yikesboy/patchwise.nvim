use std::process::{Command, Stdio};

use crate::error::PatchwiseError;
use crate::error::Result;
use crate::provider::Provider;
use crate::provider::ProviderRequest;
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

    fn generate(&self, request: &ProviderRequest) -> Result<String> {
        ensure_is_authenticated()?;
        let mut command = build_command();

        if let Some(directory) = &request.working_directory {
            command.current_dir(directory);
        }

        let prompt_bytes = request.prompt.as_str().to_owned().into_bytes();
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

    Ok(response.to_owned())
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

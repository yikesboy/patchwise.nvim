use crate::error::Result;
use std::io::Write;

use std::{
    process::{Child, ChildStdin, Command, Output, Stdio},
    thread::JoinHandle,
};

use crate::error::PatchwiseError;

pub fn run_with_stdin(
    provider: &'static str,
    mut command: Command,
    input: Vec<u8>,
) -> Result<String> {
    let mut child = spawn(provider, &mut command)?;
    let stdin_writer = spawn_stdin_writer(provider, &mut child, input)?;
    let output = wait_for_output(provider, child)?;

    join_stdin_writer(provider, stdin_writer)?;

    ensure_success(provider, &output)?;
    String::from_utf8(output.stdout)
        .map_err(|source| PatchwiseError::ProviderOutput { provider, source })
}

fn spawn(provider: &'static str, command: &mut Command) -> Result<Child> {
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| PatchwiseError::ProviderStart { provider, source })
}

fn spawn_stdin_writer(
    provider: &'static str,
    child: &mut Child,
    input: Vec<u8>,
) -> Result<JoinHandle<Result<()>>> {
    let mut stdin: ChildStdin = child
        .stdin
        .take()
        .ok_or(PatchwiseError::ProviderStdin { provider })?;

    let join_handle = std::thread::spawn(move || {
        stdin
            .write_all(&input)
            .map_err(|source| PatchwiseError::ProviderWrite { provider, source })
    });

    Ok(join_handle)
}

fn wait_for_output(provider: &'static str, child: Child) -> Result<Output> {
    child
        .wait_with_output()
        .map_err(|source| PatchwiseError::ProviderWait { provider, source })
}

fn join_stdin_writer(provider: &'static str, writer: JoinHandle<Result<()>>) -> Result<()> {
    writer
        .join()
        .unwrap_or(Err(PatchwiseError::ProviderStdinWriterPanicked {
            provider,
        }))
}

fn ensure_success(provider: &'static str, output: &Output) -> Result<()> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();

    Err(PatchwiseError::ProviderFailed {
        provider,
        status: output.status,
        stderr,
    })
}

use std::sync::OnceLock;

use tokio::runtime::{Builder, Runtime};

use crate::error::{PatchwiseError, Result};
use crate::nvim::dispatch;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn init() -> Result<()> {
    if RUNTIME.get().is_some() {
        return Ok(());
    }

    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("patchwise")
        .build()
        .map_err(PatchwiseError::RuntimeInitialization)?;

    let _ = RUNTIME.set(runtime);

    Ok(())
}

pub fn run<Work, Complete, Output>(work: Work, complete: Complete) -> Result<()>
where
    Work: FnOnce() -> Output + Send + 'static,
    Complete: FnOnce(Output) + Send + 'static,
    Output: Send + 'static,
{
    spawn_blocking(move || {
        let output = work();

        if let Err(error) = dispatch::dispatch(move || {
            complete(output);
        }) {
            eprintln!("Patchwise failed to return a background result to Neovim: {error}")
        }
    })
}

fn spawn_blocking<F>(task: F) -> Result<()>
where
    F: FnOnce() + Send + 'static,
{
    let runtime = RUNTIME.get().ok_or(PatchwiseError::RuntimeNotInitialized)?;
    let _task = runtime.spawn_blocking(task);
    Ok(())
}

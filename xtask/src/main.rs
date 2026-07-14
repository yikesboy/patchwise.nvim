/*
*   bash alternative, throw-away script. might rewrite cleanly later
*/

use std::{
    env,
    ffi::OsString,
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use anyhow::{Context, Result, bail};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("testvim: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let mut args = env::args_os().skip(1);

    let command = args
        .next()
        .context("no command provided; expected `testvim` or `health`")?;

    let neovim_args: Vec<OsString> = args.collect();

    match command.to_str() {
        Some("testvim") => run_testvim(neovim_args),
        Some("health") => run_health(),
        Some("clean") => run_clean(),
        Some(unknown) => bail!("unknown xtask command: {unknown}"),
        None => bail!("command contains invalid UTF-8"),
    }
}

fn run_clean() -> Result<()> {
    let root = project_root()?;

    let test_vim = &root.join(".testvim");
    let result = &root.join("result");

    remove_if_exists(test_vim)?;
    remove_if_exists(result)?;

    let entries =
        fs::read_dir(&root).with_context(|| format!("failed to read {}", root.display()))?;

    for entry in entries {
        let path = entry?.path();

        let is_result_link = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("result-"));

        if is_result_link {
            remove_if_exists(&path)?;
        }
    }

    println!("removed patchwise development artifacts");

    Ok(())
}

fn remove_if_exists(path: &Path) -> Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(());
        }
        Err(error) => {
            return Err(error).with_context(|| format!("failed to inspect {}", path.display()));
        }
    };

    if metadata.file_type().is_dir() {
        fs::remove_dir_all(path).with_context(|| format!("failed to remove {}", path.display()))?;
    } else {
        fs::remove_file(path).with_context(|| format!("failed to remove {}", path.display()))?;
    }

    Ok(())
}

fn run_testvim(neovim_args: Vec<OsString>) -> Result<()> {
    let project_root = project_root()?;

    build_plugin(&project_root)?;

    let library = project_root
        .join("target")
        .join("debug")
        .join("libpatchwise.so");

    if !library.is_file() {
        bail!("compiled plugin was not found at {}", library.display());
    }

    let test_root = project_root.join(".testvim");
    let runtime = test_root.join("runtime");
    let xdg_root = test_root.join("xdg");

    recreate_directory(&runtime)?;

    fs::create_dir_all(runtime.join("lua"))?;
    fs::create_dir_all(xdg_root.join("config"))?;
    fs::create_dir_all(xdg_root.join("data"))?;
    fs::create_dir_all(xdg_root.join("state"))?;
    fs::create_dir_all(xdg_root.join("cache"))?;

    let plugin_link = runtime.join("lua/patchwise.so");

    if plugin_link.exists() || plugin_link.is_symlink() {
        fs::remove_file(&plugin_link)?;
    }

    symlink(&library, &plugin_link).with_context(|| {
        format!(
            "failed to link {} to {}",
            plugin_link.display(),
            library.display()
        )
    })?;

    let runtime_command = format!("set runtimepath^={}", runtime.display());

    let status = Command::new("nvim")
        .args([
            "--clean",
            "--cmd",
            runtime_command.as_str(),
            "--cmd",
            "lua require('patchwise')",
        ])
        .args(neovim_args)
        .env("NVIM_APPNAME", "patchwise-test")
        .env("XDG_CONFIG_HOME", xdg_root.join("config"))
        .env("XDG_DATA_HOME", xdg_root.join("data"))
        .env("XDG_STATE_HOME", xdg_root.join("state"))
        .env("XDG_CACHE_HOME", xdg_root.join("cache"))
        .status()
        .context("failed to start Neovim")?;

    if !status.success() {
        bail!("Neovim exited with {status}");
    }

    Ok(())
}

fn recreate_directory(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).with_context(|| format!("failed to remove {}", path.display()))?;
    }

    fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()))?;

    Ok(())
}

fn project_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let path = String::from_utf8(output.stdout)?;
            Ok(PathBuf::from(path.trim()))
        }
        Ok(_) => {
            bail!("git rev-parse returned failure")
        }
        Err(e) => {
            bail!("git rev-parse subcommand invocation failed {e}")
        }
    }
}

fn build_plugin(project_root: &Path) -> Result<()> {
    let status = Command::new("cargo")
        .args(["build", "--package", "patchwise"])
        .current_dir(project_root)
        .status()
        .context("failed to run cargo build")?;

    if !status.success() {
        bail!("patchwise.nvim build failed");
    }

    Ok(())
}

fn run_health() -> Result<()> {
    check_command("cargo", &["--version"])?;
    check_command("nvim", &["--version"])?;
    check_command("codex", &["--version"])?;

    println!("patchwise.nvim development environment is healthy");
    Ok(())
}

fn check_command(command: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(command)
        .args(args)
        .status()
        .with_context(|| format!("failed to start {command}"))?;

    if !status.success() {
        bail!("{command} returned {status}");
    }

    Ok(())
}

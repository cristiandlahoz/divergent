//! Divergent's terminal entrypoint.
//!
//! The binary owns the UI-heavy parts of the program because they are shaped by
//! terminal state, local VCS discovery, and `gh` integration. Reusable parsing
//! contracts live in the library target so their examples can run as doctests.

use clap::Parser;
use command::diff::theme::ThemeChoice;
use config::cli::{resolve_theme, Cli, Commands};
use divergent::commit_reference;
use error::DivergentError;
use std::process;
use std::str::FromStr;
use vcs::VcsBackendType;

mod command;
mod config;
mod error;
mod vcs;

/// Starts Divergent as a focused diff viewer.
///
/// The binary does one thing: resolve a version-control backend, collect the
/// requested diff target, and hand the rest of the session to the terminal UI.
fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[91m\rerror:\x1b[0m {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), DivergentError> {
    validate_theme_env()?;
    let cli = Cli::parse();

    let cwd = std::env::current_dir()?;
    let vcs_override = cli.vcs.map(VcsBackendType::from);
    let backend = vcs::get_backend(&cwd, vcs_override)?;

    match cli.command {
        Commands::Diff {
            reference,
            pr,
            file,
            watch,
            stacked,
            focus,
            theme,
        } => {
            let theme = resolve_theme(theme).map_err(DivergentError::InvalidInput)?;
            let options = command::diff::DiffOptions {
                reference,
                pr,
                file,
                watch,
                stacked,
                focus,
                theme,
            };
            command::diff::run_diff_ui(options, backend.as_ref())?;
        }
    }

    Ok(())
}

fn validate_theme_env() -> Result<(), DivergentError> {
    if std::env::var_os("DIVERGENT_THEME").is_none() || args_include_theme_override() {
        return Ok(());
    }

    let value = std::env::var("DIVERGENT_THEME")
        .map_err(|e| DivergentError::InvalidInput(format!("invalid DIVERGENT_THEME: {e}")))?;
    ThemeChoice::from_str(&value)
        .map(|_| ())
        .map_err(DivergentError::InvalidInput)
}

fn args_include_theme_override() -> bool {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--theme" {
            return args.next().is_some();
        }
        if arg.starts_with("--theme=") {
            return true;
        }
    }
    false
}

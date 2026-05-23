//! Divergent's terminal entrypoint.
//!
//! The binary owns the UI-heavy parts of the program because they are shaped by
//! terminal state, local VCS discovery, and `gh` integration. Reusable parsing
//! contracts live in the library target so their examples can run as doctests.

use clap::Parser;
use command::diff::theme::ThemeChoice;
use config::cli::{resolve_theme, Cli, Commands};
use error::DivergentError;
use std::process;
use std::str::FromStr;
use vcs::VcsBackendType;

mod command;
mod commit_reference;
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
            let backend = selected_backend(cli.vcs)?;
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
        Commands::Git { command } => {
            command::git_integration::execute(command)?;
        }
        Commands::GitPager { patch_file } => {
            let backend = selected_backend(cli.vcs)?;
            let theme = resolve_theme(None).map_err(DivergentError::InvalidInput)?;
            let options = command::diff::DiffOptions {
                reference: None,
                pr: None,
                file: None,
                watch: false,
                stacked: false,
                focus: None,
                theme,
            };
            let mut stdin = std::io::stdin();
            command::git_integration::run_pager(
                options,
                backend.as_ref(),
                &mut stdin,
                patch_file.as_deref(),
            )?;
        }
    }

    Ok(())
}

fn selected_backend(
    vcs_override: Option<config::cli::VcsOverride>,
) -> Result<Box<dyn vcs::VcsBackend>, DivergentError> {
    let cwd = std::env::current_dir()?;
    let vcs_override = vcs_override.map(VcsBackendType::from);
    Ok(vcs::get_backend(&cwd, vcs_override)?)
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

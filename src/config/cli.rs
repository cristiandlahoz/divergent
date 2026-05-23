use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::str::FromStr;

use crate::command::diff::theme::ThemeChoice;
use crate::commit_reference::CommitReference;

/// Selects a repository backend when auto-detection is not the right call.
#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum VcsOverride {
    /// Use the git backend.
    Git,
    /// Use the jj (Jujutsu) backend.
    Jj,
}

#[derive(Parser)]
#[command(name = "divergent")]
#[command(about = "Fast terminal diff viewer for git and jj", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Version control system to use (auto-detected if not specified).
    #[arg(value_enum, long = "vcs")]
    pub vcs: Option<VcsOverride>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch the interactive diff viewer.
    Diff {
        /// Commit reference: SHA, HEAD, HEAD~3..HEAD, main..feature, main...feature.
        #[arg(value_parser = clap::value_parser!(CommitReference))]
        reference: Option<CommitReference>,

        /// View a GitHub pull request (number or URL).
        #[arg(long)]
        pr: Option<String>,

        /// Filter to specific files.
        #[arg(short, long)]
        file: Option<Vec<String>>,

        /// Watch for file changes and auto-reload.
        #[arg(short, long)]
        watch: bool,

        /// Show commits stacked (commit-by-commit navigation with ctrl+l/h).
        #[arg(long)]
        stacked: bool,

        /// Initially focus on this file path.
        #[arg(long)]
        focus: Option<String>,

        /// Color theme to use. CLI value overrides DIVERGENT_THEME.
        #[arg(long, value_enum)]
        theme: Option<ThemeChoice>,
    },
    /// Manage global git integration.
    Git {
        #[command(subcommand)]
        command: GitCommand,
    },
    #[command(name = "git-pager", hide = true)]
    GitPager {
        #[arg(long)]
        patch_file: Option<PathBuf>,
    },
}

pub fn resolve_theme(cli_theme: Option<ThemeChoice>) -> Result<ThemeChoice, String> {
    if let Some(theme) = cli_theme {
        return Ok(theme);
    }

    match std::env::var("DIVERGENT_THEME") {
        Ok(value) => <ThemeChoice as FromStr>::from_str(&value),
        Err(std::env::VarError::NotPresent) => Ok(ThemeChoice::Midnight),
        Err(err) => Err(format!("invalid DIVERGENT_THEME: {err}")),
    }
}

#[derive(Subcommand)]
pub enum GitCommand {
    /// Configure global git diff to open Divergent.
    Install {
        /// Overwrite an existing non-Divergent pager.diff value.
        #[arg(long)]
        force: bool,

        /// Divergent binary path to install into the pager command.
        #[arg(long)]
        binary: Option<PathBuf>,
    },
    /// Remove the Divergent-managed global git diff integration.
    Uninstall,
    /// Show whether global git diff is managed by Divergent.
    Status,
    /// Explain whether Git will actually route diffs through Divergent.
    Doctor,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    #[test]
    fn test_vcs_git_parses() {
        let cli = Cli::try_parse_from(["divergent", "--vcs", "git", "diff"]).unwrap();
        assert_eq!(cli.vcs, Some(VcsOverride::Git));
    }

    #[test]
    fn test_vcs_jj_parses() {
        let cli = Cli::try_parse_from(["divergent", "--vcs", "jj", "diff"]).unwrap();
        assert_eq!(cli.vcs, Some(VcsOverride::Jj));
    }

    #[test]
    fn test_git_install_parses() {
        let cli = Cli::try_parse_from(["divergent", "git", "install", "--force"]).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Git {
                command: GitCommand::Install {
                    force: true,
                    binary: None
                }
            }
        ));
    }

    #[test]
    fn test_theme_cli_parses() {
        let cli = Cli::try_parse_from(["divergent", "diff", "--theme", "midnight"]).unwrap();
        let Commands::Diff { theme, .. } = cli.command else {
            panic!("expected diff command");
        };
        assert_eq!(theme, Some(ThemeChoice::Midnight));
    }

    #[test]
    fn test_theme_env_parses() {
        let guard = EnvGuard::set("DIVERGENT_THEME", "tokyonight");
        let theme = resolve_theme(None).unwrap();
        assert_eq!(theme, ThemeChoice::Tokyonight);
        drop(guard);
    }

    #[test]
    fn test_theme_default_is_midnight() {
        let guard = EnvGuard::unset("DIVERGENT_THEME");
        let theme = resolve_theme(None).unwrap();
        assert_eq!(theme, ThemeChoice::Midnight);
        drop(guard);
    }

    #[test]
    fn test_theme_cli_overrides_env() {
        let guard = EnvGuard::set("DIVERGENT_THEME", "tokyonight");
        let theme = resolve_theme(Some(ThemeChoice::Midnight)).unwrap();
        assert_eq!(theme, ThemeChoice::Midnight);
        drop(guard);
    }

    #[test]
    fn test_invalid_theme_env_fails() {
        let guard = EnvGuard::set("DIVERGENT_THEME", "bad");
        let err = resolve_theme(None).unwrap_err();
        assert!(err.contains("invalid theme"));
        drop(guard);
    }

    struct EnvGuard {
        key: &'static str,
        previous: Option<String>,
        _lock: MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let lock = ENV_LOCK
                .get_or_init(|| Mutex::new(()))
                .lock()
                .expect("env test lock poisoned");
            let previous = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self {
                key,
                previous,
                _lock: lock,
            }
        }

        fn unset(key: &'static str) -> Self {
            let lock = ENV_LOCK
                .get_or_init(|| Mutex::new(()))
                .lock()
                .expect("env test lock poisoned");
            let previous = std::env::var(key).ok();
            std::env::remove_var(key);
            Self {
                key,
                previous,
                _lock: lock,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.previous {
                std::env::set_var(self.key, value);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }
}

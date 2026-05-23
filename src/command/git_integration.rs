use std::ffi::OsStr;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::command::diff::{self, DiffOptions};
use crate::config::cli::GitCommand;
use crate::error::LumenError;
use crate::vcs::VcsBackend;

const PAGER_KEY: &str = "pager.diff";
pub fn execute(command: GitCommand) -> Result<(), LumenError> {
    match command {
        GitCommand::Install { force, binary } => install(binary, force),
        GitCommand::Uninstall => uninstall(),
        GitCommand::Status => status(),
    }
}

pub fn install(binary: Option<PathBuf>, force: bool) -> Result<(), LumenError> {
    let binary = binary.unwrap_or(std::env::current_exe()?);
    let current = read_pager_value(None)?;

    if let Some(value) = current.as_deref() {
        if !is_managed(value) && !force {
            return Err(LumenError::CommandError(format!(
                "{} is already set to '{}'. Re-run with --force to replace it.",
                PAGER_KEY, value
            )));
        }
    }

    write_pager_value(None, &managed_pager_command(&binary))?;
    println!("installed divergent as the global git diff pager");
    Ok(())
}

pub fn uninstall() -> Result<(), LumenError> {
    match read_pager_value(None)? {
        Some(value) if is_managed(&value) => {
            unset_pager_value(None)?;
            println!("removed divergent global git diff pager");
            Ok(())
        }
        Some(value) => Err(LumenError::CommandError(format!(
            "{} is not managed by Divergent: '{}'",
            PAGER_KEY, value
        ))),
        None => {
            println!("divergent is not installed as the global git diff pager");
            Ok(())
        }
    }
}

pub fn status() -> Result<(), LumenError> {
    match read_pager_value(None)? {
        Some(value) if is_managed(&value) => println!("installed: {}", value),
        Some(value) => println!("conflict: {} is '{}'", PAGER_KEY, value),
        None => println!("not installed"),
    }
    Ok(())
}

pub fn run_pager(
    options: DiffOptions,
    backend: &dyn VcsBackend,
    input: &mut dyn Read,
) -> io::Result<()> {
    let mut patch = String::new();
    input.read_to_string(&mut patch)?;
    if !attach_tty_for_tui()? {
        io::stdout().write_all(patch.as_bytes())?;
        return Ok(());
    }
    let file_diffs = parse_git_patch(&patch);
    diff::run_diff_ui_from_file_diffs(options, file_diffs, backend)
}

fn managed_pager_command(binary: &Path) -> String {
    format!(
        "sh -c 'exec \"$0\" git-pager' {}",
        shell_quote(binary.as_os_str())
    )
}

fn shell_quote(value: &OsStr) -> String {
    let value = value.to_string_lossy();
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn is_managed(value: &str) -> bool {
    value.contains("divergent") && value.contains("git-pager")
}

fn read_pager_value(git_config_global: Option<&Path>) -> Result<Option<String>, LumenError> {
    let output = git_command(git_config_global)
        .args(["config", "--global", "--get", PAGER_KEY])
        .output()?;
    if output.status.success() {
        let value = String::from_utf8(output.stdout)?.trim().to_string();
        Ok((!value.is_empty()).then_some(value))
    } else {
        Ok(None)
    }
}

fn write_pager_value(git_config_global: Option<&Path>, value: &str) -> Result<(), LumenError> {
    let status = git_command(git_config_global)
        .args(["config", "--global", PAGER_KEY, value])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(LumenError::CommandError(format!(
            "failed to write global {}",
            PAGER_KEY
        )))
    }
}

fn unset_pager_value(git_config_global: Option<&Path>) -> Result<(), LumenError> {
    let status = git_command(git_config_global)
        .args(["config", "--global", "--unset", PAGER_KEY])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(LumenError::CommandError(format!(
            "failed to unset global {}",
            PAGER_KEY
        )))
    }
}

fn git_command(git_config_global: Option<&Path>) -> Command {
    let mut command = Command::new("git");
    command.current_dir("/");
    if let Some(path) = git_config_global {
        command.env("GIT_CONFIG_GLOBAL", path);
    }
    command
}

fn parse_git_patch(patch: &str) -> Vec<diff::FileDiff> {
    let mut files = Vec::new();
    let mut current: Option<PatchFile> = None;

    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            if let Some(file) = current.take() {
                files.push(file.into_file_diff());
            }
            current = Some(PatchFile::new(rest));
            continue;
        }

        let Some(file) = current.as_mut() else {
            continue;
        };

        if line == "GIT binary patch" || line.starts_with("Binary files ") {
            file.is_binary = true;
        } else if line == "new file mode" || line.starts_with("new file mode ") {
            file.status = diff::FileStatus::Added;
        } else if line == "deleted file mode" || line.starts_with("deleted file mode ") {
            file.status = diff::FileStatus::Deleted;
        } else if let Some(path) = line.strip_prefix("--- a/") {
            file.old_path = Some(path.to_string());
        } else if let Some(path) = line.strip_prefix("+++ b/") {
            file.new_path = Some(path.to_string());
        } else if line.starts_with("@@") {
            file.in_hunk = true;
        } else if file.in_hunk {
            match line.as_bytes().first().copied() {
                Some(b' ') => {
                    let text = line[1..].to_string();
                    file.old_content.push(text.clone());
                    file.new_content.push(text);
                }
                Some(b'-') if !line.starts_with("--- ") => {
                    file.old_content.push(line[1..].to_string());
                }
                Some(b'+') if !line.starts_with("+++ ") => {
                    file.new_content.push(line[1..].to_string());
                }
                _ => {}
            }
        }
    }

    if let Some(file) = current {
        files.push(file.into_file_diff());
    }

    files
}

struct PatchFile {
    old_path: Option<String>,
    new_path: Option<String>,
    old_content: Vec<String>,
    new_content: Vec<String>,
    status: diff::FileStatus,
    is_binary: bool,
    in_hunk: bool,
}

impl PatchFile {
    fn new(diff_git_paths: &str) -> Self {
        let mut parts = diff_git_paths.split_whitespace();
        let old_path = parts.next().and_then(|path| path.strip_prefix("a/"));
        let new_path = parts.next().and_then(|path| path.strip_prefix("b/"));
        Self {
            old_path: old_path.map(ToString::to_string),
            new_path: new_path.map(ToString::to_string),
            old_content: Vec::new(),
            new_content: Vec::new(),
            status: diff::FileStatus::Modified,
            is_binary: false,
            in_hunk: false,
        }
    }

    fn into_file_diff(self) -> diff::FileDiff {
        let filename = self
            .new_path
            .filter(|path| path != "/dev/null")
            .or(self.old_path)
            .unwrap_or_else(|| "unknown".to_string());
        diff::FileDiff {
            filename,
            old_content: self.old_content.join("\n"),
            new_content: self.new_content.join("\n"),
            status: self.status,
            is_binary: self.is_binary,
        }
    }
}

#[cfg(unix)]
fn attach_tty_for_tui() -> io::Result<bool> {
    use std::fs::File;
    use std::os::fd::AsRawFd;

    if unsafe { libc::isatty(libc::STDOUT_FILENO) != 1 } {
        return Ok(false);
    }

    let Ok(tty) = File::open("/dev/tty") else {
        return Ok(false);
    };
    let result = unsafe { libc::dup2(tty.as_raw_fd(), libc::STDIN_FILENO) };
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(true)
    }
}

#[cfg(not(unix))]
fn attach_tty_for_tui() -> io::Result<bool> {
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_global_config() -> tempfile::NamedTempFile {
        tempfile::NamedTempFile::new().expect("temp config")
    }

    #[test]
    fn install_writes_managed_pager() {
        let config = temp_global_config();
        write_pager_value(
            Some(config.path()),
            &managed_pager_command(Path::new("/tmp/divergent")),
        )
        .unwrap();
        let value = read_pager_value(Some(config.path())).unwrap().unwrap();
        assert!(is_managed(&value));
        assert!(value.contains("/tmp/divergent"));
    }

    #[test]
    fn uninstall_only_removes_managed_value() {
        let config = temp_global_config();
        write_pager_value(Some(config.path()), "less").unwrap();
        assert!(!is_managed(
            &read_pager_value(Some(config.path())).unwrap().unwrap()
        ));

        write_pager_value(
            Some(config.path()),
            &managed_pager_command(Path::new("/bin/divergent")),
        )
        .unwrap();
        unset_pager_value(Some(config.path())).unwrap();
        assert!(read_pager_value(Some(config.path())).unwrap().is_none());
    }

    #[test]
    fn parses_basic_git_patch() {
        let patch = r#"diff --git a/application.yml b/application.yml
index 1111111..2222222 100644
--- a/application.yml
+++ b/application.yml
@@ -1,2 +1,2 @@
 name: old
-port: 8080
+port: 9090
"#;
        let files = parse_git_patch(patch);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filename, "application.yml");
        assert!(files[0].old_content.contains("8080"));
        assert!(files[0].new_content.contains("9090"));
    }
}

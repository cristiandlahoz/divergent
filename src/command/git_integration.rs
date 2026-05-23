use std::ffi::OsStr;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::command::diff::{self, DiffOptions};
use crate::config::cli::GitCommand;
use crate::error::DivergentError;
use crate::vcs::VcsBackend;

const PAGER_KEY: &str = "pager.diff";
const CORE_PAGER_KEY: &str = "core.pager";

pub fn execute(command: GitCommand) -> Result<(), DivergentError> {
    match command {
        GitCommand::Install { force, binary } => install(binary, force),
        GitCommand::Uninstall => uninstall(),
        GitCommand::Status => status(),
        GitCommand::Doctor => doctor(),
    }
}

pub fn install(binary: Option<PathBuf>, force: bool) -> Result<(), DivergentError> {
    let binary = binary.unwrap_or(std::env::current_exe()?);
    let current = read_pager_value(None)?;

    if let Some(value) = current.as_deref() {
        if !is_managed(value) && !force {
            return Err(DivergentError::InvalidInput(format!(
                "{} is already set to '{}'. Re-run with --force to replace it.",
                PAGER_KEY, value
            )));
        }
    }

    write_pager_value(None, &managed_pager_command(&binary))?;
    println!("installed divergent as the global git diff pager");
    print_env_warning();
    Ok(())
}

pub fn uninstall() -> Result<(), DivergentError> {
    match read_pager_value(None)? {
        Some(value) if is_managed(&value) => {
            unset_pager_value(None)?;
            println!("removed divergent global git diff pager");
            Ok(())
        }
        Some(value) => Err(DivergentError::InvalidInput(format!(
            "{} is not managed by Divergent: '{}'",
            PAGER_KEY, value
        ))),
        None => {
            println!("divergent is not installed as the global git diff pager");
            Ok(())
        }
    }
}

pub fn status() -> Result<(), DivergentError> {
    match read_pager_value(None)? {
        Some(value) if is_managed(&value) => println!("installed: {}", value),
        Some(value) => println!("conflict: {} is '{}'", PAGER_KEY, value),
        None => println!("not installed"),
    }
    print_env_warning();
    Ok(())
}

pub fn doctor() -> Result<(), DivergentError> {
    let pager_diff = read_pager_value(None)?;
    let core_pager = read_global_value(CORE_PAGER_KEY, None)?;
    let git_pager = std::env::var("GIT_PAGER").ok();
    let pager = std::env::var("PAGER").ok();
    let env_conflicts = env_pager_conflicts(git_pager.as_deref(), pager.as_deref());

    println!(
        "pager.diff: {}",
        pager_diff.as_deref().unwrap_or("<not set>")
    );
    println!(
        "core.pager: {}",
        core_pager.as_deref().unwrap_or("<not set>")
    );
    println!("GIT_PAGER: {}", git_pager.as_deref().unwrap_or("<not set>"));
    println!("PAGER: {}", pager.as_deref().unwrap_or("<not set>"));
    println!(
        "divergent pager.diff: {}",
        pager_diff.as_deref().is_some_and(is_managed)
    );
    println!("env may bypass paging: {}", !env_conflicts.is_empty());
    println!("tty available: {}", tty_available());
    for conflict in env_conflicts {
        println!("warning: {}", conflict);
    }
    Ok(())
}

pub fn run_pager(
    options: DiffOptions,
    backend: &dyn VcsBackend,
    input: &mut dyn Read,
    patch_file: Option<&Path>,
) -> io::Result<()> {
    let patch = read_patch(input, patch_file)?;
    if !attach_tty_for_tui()? {
        io::stdout().write_all(patch.as_bytes())?;
        return Ok(());
    }
    let file_diffs = parse_git_patch(&patch);
    diff::run_diff_ui_from_file_diffs(options, file_diffs, backend)
}

fn managed_pager_command(binary: &Path) -> String {
    format!(
        "sh -c 'tmp=$(mktemp \"${{TMPDIR:-/tmp}}/divergent-git-pager.XXXXXX\") || exit 1; trap \"rm -f \\\"$tmp\\\"\" EXIT HUP INT TERM; cat > \"$tmp\"; if [ -r /dev/tty ] && [ -w /dev/tty ]; then exec \"$0\" git-pager --patch-file \"$tmp\" < /dev/tty > /dev/tty 2> /dev/tty; else exec \"$0\" git-pager --patch-file \"$tmp\"; fi' {}",
        shell_quote(binary.as_os_str())
    )
}

fn read_patch(input: &mut dyn Read, patch_file: Option<&Path>) -> io::Result<String> {
    if let Some(path) = patch_file {
        let patch = fs::read_to_string(path)?;
        let _ = fs::remove_file(path);
        Ok(patch)
    } else {
        let mut patch = String::new();
        input.read_to_string(&mut patch)?;
        Ok(patch)
    }
}

fn shell_quote(value: &OsStr) -> String {
    let value = value.to_string_lossy();
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn is_managed(value: &str) -> bool {
    value.contains("divergent") && value.contains("git-pager")
}

fn read_pager_value(git_config_global: Option<&Path>) -> Result<Option<String>, DivergentError> {
    read_global_value(PAGER_KEY, git_config_global)
}

fn read_global_value(
    key: &str,
    git_config_global: Option<&Path>,
) -> Result<Option<String>, DivergentError> {
    let output = git_command(git_config_global)
        .args(["config", "--global", "--get", key])
        .output()?;
    if output.status.success() {
        let value = String::from_utf8(output.stdout)?.trim().to_string();
        Ok((!value.is_empty()).then_some(value))
    } else {
        Ok(None)
    }
}

fn print_env_warning() {
    for conflict in env_pager_conflicts(
        std::env::var("GIT_PAGER").ok().as_deref(),
        std::env::var("PAGER").ok().as_deref(),
    ) {
        eprintln!("\x1b[33mwarning:\x1b[0m {}", conflict);
    }
}

fn env_pager_conflicts(git_pager: Option<&str>, pager: Option<&str>) -> Vec<String> {
    let mut conflicts = Vec::new();
    if let Some(value) = git_pager.filter(|value| is_conflicting_pager_value(value)) {
        conflicts.push(format!(
            "GIT_PAGER='{}' can override Git pager config; unset it when using Divergent",
            value
        ));
    }
    if let Some(value) = pager.filter(|value| is_conflicting_pager_value(value)) {
        conflicts.push(format!(
            "PAGER='{}' can affect Git pager selection; unset it if git diff bypasses Divergent",
            value
        ));
    }
    conflicts
}

fn is_conflicting_pager_value(value: &str) -> bool {
    let value = value.trim().to_ascii_lowercase();
    !value.is_empty()
        && !value.contains("divergent")
        && (value == "cat" || value == "less" || value.contains("hunk") || value.contains("delta"))
}

fn write_pager_value(git_config_global: Option<&Path>, value: &str) -> Result<(), DivergentError> {
    let status = git_command(git_config_global)
        .args(["config", "--global", PAGER_KEY, value])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(DivergentError::InvalidInput(format!(
            "failed to write global {}",
            PAGER_KEY
        )))
    }
}

fn unset_pager_value(git_config_global: Option<&Path>) -> Result<(), DivergentError> {
    let status = git_command(git_config_global)
        .args(["config", "--global", "--unset", PAGER_KEY])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(DivergentError::InvalidInput(format!(
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
    use std::fs::OpenOptions;
    use std::os::fd::AsRawFd;

    let Ok(tty) = OpenOptions::new().read(true).write(true).open("/dev/tty") else {
        return Ok(false);
    };

    for fd in [libc::STDIN_FILENO, libc::STDOUT_FILENO, libc::STDERR_FILENO] {
        if unsafe { libc::dup2(tty.as_raw_fd(), fd) } == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(true)
}

#[cfg(not(unix))]
fn attach_tty_for_tui() -> io::Result<bool> {
    Ok(true)
}

#[cfg(unix)]
fn tty_available() -> bool {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .is_ok()
}

#[cfg(not(unix))]
fn tty_available() -> bool {
    true
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
    fn managed_pager_command_uses_tty_tempfile_and_cleanup() {
        let command = managed_pager_command(Path::new("/tmp/divergent"));
        assert!(command.contains("mktemp"));
        assert!(command.contains("trap"));
        assert!(command.contains("/dev/tty"));
        assert!(command.contains("--patch-file"));
    }

    #[test]
    fn env_conflicts_detect_cat_hunk_and_delta() {
        let conflicts = env_pager_conflicts(Some("cat"), Some("hunk pager"));
        assert_eq!(conflicts.len(), 2);

        let conflicts = env_pager_conflicts(Some("delta --color-only"), Some("divergent"));
        assert_eq!(conflicts.len(), 1);
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

//! Guarded script execution for scaffold.
//!
//! This crate provides safe execution of shell commands with
//! network policy enforcement and command filtering.

use manifest::NetworkPolicy;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use thiserror::Error;

/// Errors that can occur during script execution.
#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("command blocked by network policy: {0}")]
    NetworkBlocked(String),

    #[error("command execution failed: {0}")]
    ExecutionFailed(String),

    #[error("script execution not allowed (use --allow-scripts)")]
    ScriptsNotAllowed,
}

/// Result type for executor operations.
pub type Result<T> = std::result::Result<T, ExecutorError>;

/// Known commands that perform network operations.
const NETWORK_COMMANDS: &[&str] = &[
    "curl",
    "wget",
    "fetch",
    "npm install",
    "npm i",
    "npm ci",
    "yarn install",
    "yarn add",
    "pnpm install",
    "pnpm add",
    "pip install",
    "pip3 install",
    "cargo install",
    "cargo add",
    "go get",
    "go install",
    "gem install",
    "bundle install",
    "composer install",
    "apt install",
    "apt-get install",
    "brew install",
    "git clone",
    "git fetch",
    "git pull",
    "git push",
];

/// Executor configuration.
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Network policy.
    pub network_policy: NetworkPolicy,
    /// Whether scripts are allowed.
    pub allow_scripts: bool,
    /// Whether to capture output.
    pub capture_output: bool,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            network_policy: NetworkPolicy::Allow,
            allow_scripts: false,
            capture_output: true,
        }
    }
}

/// Command executor with safety guards.
pub struct Executor {
    config: ExecutorConfig,
}

impl Executor {
    /// Create a new executor with the given configuration.
    pub fn new(config: ExecutorConfig) -> Self {
        Self { config }
    }

    /// Check if a command involves network access.
    pub fn is_network_command(cmd: &str) -> bool {
        let cmd_lower = cmd.to_lowercase();
        NETWORK_COMMANDS.iter().any(|nc| cmd_lower.contains(nc))
    }

    /// Check if a command is allowed under the current policy.
    pub fn is_command_allowed(&self, cmd: &str) -> Result<()> {
        if !self.config.allow_scripts {
            return Err(ExecutorError::ScriptsNotAllowed);
        }

        match self.config.network_policy {
            NetworkPolicy::Allow => Ok(()),
            NetworkPolicy::Deny => {
                if Self::is_network_command(cmd) {
                    Err(ExecutorError::NetworkBlocked(cmd.to_string()))
                } else {
                    Ok(())
                }
            }
            NetworkPolicy::Ask => {
                // In non-interactive mode, treat as deny
                // Interactive prompting is handled at CLI level
                if Self::is_network_command(cmd) {
                    Err(ExecutorError::NetworkBlocked(cmd.to_string()))
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Execute a shell command.
    pub fn execute(&self, cmd: &str, workdir: Option<&Path>) -> Result<Output> {
        self.is_command_allowed(cmd)?;

        let mut command = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.args(["/C", cmd]);
            c
        } else {
            let mut c = Command::new("sh");
            c.args(["-c", cmd]);
            c
        };

        if let Some(dir) = workdir {
            command.current_dir(dir);
        }

        if self.config.capture_output {
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
        }

        let output = command.spawn()?.wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ExecutorError::ExecutionFailed(format!(
                "command '{}' failed with exit code {:?}: {}",
                cmd,
                output.status.code(),
                stderr
            )));
        }

        Ok(output)
    }

    /// Execute a command and return stdout as string.
    pub fn execute_capture(&self, cmd: &str, workdir: Option<&Path>) -> Result<String> {
        let output = self.execute(cmd, workdir)?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Run test commands.
    pub fn run_tests(&self, tests: &[String], workdir: &Path) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        for test in tests {
            let result = match self.execute(test, Some(workdir)) {
                Ok(output) => TestResult {
                    command: test.clone(),
                    success: true,
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                },
                Err(e) => TestResult {
                    command: test.clone(),
                    success: false,
                    stdout: String::new(),
                    stderr: e.to_string(),
                },
            };
            results.push(result);
        }

        Ok(results)
    }
}

/// Result of a test command execution.
#[derive(Debug, Clone)]
pub struct TestResult {
    /// The command that was run.
    pub command: String,
    /// Whether the command succeeded.
    pub success: bool,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_network_command() {
        assert!(Executor::is_network_command("curl https://example.com"));
        assert!(Executor::is_network_command("npm install express"));
        assert!(Executor::is_network_command("pip install requests"));
        assert!(!Executor::is_network_command("cargo build"));
        assert!(!Executor::is_network_command("echo hello"));
    }

    #[test]
    fn test_network_policy_deny() {
        let executor = Executor::new(ExecutorConfig {
            network_policy: NetworkPolicy::Deny,
            allow_scripts: true,
            capture_output: true,
        });

        assert!(executor.is_command_allowed("echo hello").is_ok());
        assert!(executor
            .is_command_allowed("curl https://example.com")
            .is_err());
    }

    #[test]
    fn test_scripts_not_allowed() {
        let executor = Executor::new(ExecutorConfig {
            network_policy: NetworkPolicy::Allow,
            allow_scripts: false,
            capture_output: true,
        });

        assert!(executor.is_command_allowed("echo hello").is_err());
    }
}

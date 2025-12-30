//! JSON output adapter for opencode UI integration.
//!
//! This crate provides structured JSON output for scaffold
//! to integrate with opencode's command palette and UI.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur in opencode adapter.
#[derive(Debug, Error)]
pub enum OpenCodeError {
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type for opencode operations.
pub type Result<T> = std::result::Result<T, OpenCodeError>;

/// Status of an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error,
    Warning,
    Info,
}

/// A file change to be displayed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    /// File path.
    pub path: PathBuf,
    /// Change type.
    pub change_type: ChangeType,
    /// Optional diff content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

/// Type of file change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
    Unchanged,
}

/// Template information for listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// Template ID.
    pub id: String,
    /// Template name.
    pub name: String,
    /// Template description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Template version.
    pub version: String,
    /// Target language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed.
    pub valid: bool,
    /// Validation errors.
    #[serde(default)]
    pub errors: Vec<String>,
    /// Validation warnings.
    #[serde(default)]
    pub warnings: Vec<String>,
}

/// Generation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResult {
    /// Operation status.
    pub status: Status,
    /// Output directory.
    pub output_dir: PathBuf,
    /// Files that were/would be changed.
    pub files: Vec<FileChange>,
    /// Whether this was a dry run.
    pub dry_run: bool,
    /// Optional message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Test result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOutput {
    /// Command that was run.
    pub command: String,
    /// Whether the test passed.
    pub passed: bool,
    /// Test output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Error message if failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Test run result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunResult {
    /// Operation status.
    pub status: Status,
    /// Test results.
    pub tests: Vec<TestOutput>,
    /// Total tests run.
    pub total: usize,
    /// Tests passed.
    pub passed: usize,
    /// Tests failed.
    pub failed: usize,
}

/// JSON output wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Output {
    /// List of templates.
    TemplateList { templates: Vec<TemplateInfo> },
    /// Validation result.
    Validation(ValidationResult),
    /// Generation result.
    Generate(GenerateResult),
    /// Test run result.
    TestRun(TestRunResult),
    /// Error output.
    Error { message: String, code: Option<i32> },
}

impl Output {
    /// Convert to JSON string.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Convert to compact JSON string.
    pub fn to_json_compact(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Create an error output.
    pub fn error(message: impl Into<String>) -> Self {
        Output::Error {
            message: message.into(),
            code: None,
        }
    }

    /// Create an error output with code.
    pub fn error_with_code(message: impl Into<String>, code: i32) -> Self {
        Output::Error {
            message: message.into(),
            code: Some(code),
        }
    }
}

/// Print JSON output to stdout.
pub fn print_json(output: &Output) -> Result<()> {
    println!("{}", output.to_json()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_serialization() {
        let output = Output::TemplateList {
            templates: vec![TemplateInfo {
                id: "rust-basic".to_string(),
                name: "Rust Basic".to_string(),
                description: Some("A basic Rust project".to_string()),
                version: "0.1.0".to_string(),
                language: Some("rust".to_string()),
            }],
        };

        let json = output.to_json().unwrap();
        assert!(json.contains("rust-basic"));
        assert!(json.contains("template_list"));
    }

    #[test]
    fn test_error_output() {
        let output = Output::error("something went wrong");
        let json = output.to_json().unwrap();
        assert!(json.contains("something went wrong"));
        assert!(json.contains("error"));
    }
}

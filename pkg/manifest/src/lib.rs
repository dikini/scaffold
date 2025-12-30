//! Manifest types and validation for scaffold templates.
//!
//! This crate provides the core data structures for scaffold template manifests,
//! including parsing, validation, and condition evaluation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur when working with manifests.
#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("failed to read manifest file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("failed to parse manifest YAML: {0}")]
    ParseError(#[from] serde_yaml::Error),

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("invalid condition: {0}")]
    ConditionError(String),
}

/// Result type for manifest operations.
pub type Result<T> = std::result::Result<T, ManifestError>;

/// Network policy for template execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkPolicy {
    /// Allow network access (default).
    #[default]
    Allow,
    /// Deny network access.
    Deny,
    /// Ask user before allowing network access.
    Ask,
}

/// CI provider for generated projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CiProvider {
    /// GitHub Actions (default).
    #[default]
    GithubActions,
    /// GitLab CI.
    GitlabCi,
    /// No CI configuration.
    None,
}

/// Input type for template variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    /// String input (default).
    #[default]
    String,
    /// Boolean input.
    Bool,
    /// Integer input.
    Int,
    /// Selection from enum values.
    Enum,
}

/// A template input definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// Input variable name.
    pub name: String,

    /// Input type.
    #[serde(default, rename = "type")]
    pub input_type: InputType,

    /// Default value (as string, will be parsed according to type).
    #[serde(default)]
    pub default: Option<String>,

    /// Prompt to show user.
    #[serde(default)]
    pub prompt: Option<String>,

    /// Whether input is required.
    #[serde(default)]
    pub required: bool,

    /// Enum values (for InputType::Enum).
    #[serde(default)]
    pub enum_values: Vec<String>,
}

/// Step types in a template manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Step {
    /// Render a template file.
    RenderTemplate {
        /// Source template path (relative to template dir).
        src: String,
        /// Destination path (supports template variables).
        dest: String,
        /// Optional condition for this step.
        #[serde(default)]
        when: Option<String>,
    },

    /// Create a file with inline content.
    CreateFile {
        /// Destination path.
        dest: String,
        /// File content (supports template variables).
        content: String,
        /// Optional condition.
        #[serde(default)]
        when: Option<String>,
    },

    /// Copy a file or directory.
    Copy {
        /// Source path.
        src: String,
        /// Destination path.
        dest: String,
        /// Optional condition.
        #[serde(default)]
        when: Option<String>,
    },

    /// Run a shell command.
    RunShell {
        /// Command to run.
        cmd: String,
        /// Working directory (defaults to output dir).
        #[serde(default)]
        workdir: Option<String>,
        /// Optional condition.
        #[serde(default)]
        when: Option<String>,
    },

    /// Apply a patch file.
    ApplyPatch {
        /// Patch file path.
        patch: String,
        /// Optional condition.
        #[serde(default)]
        when: Option<String>,
    },
}

impl Step {
    /// Get the condition for this step, if any.
    pub fn condition(&self) -> Option<&str> {
        match self {
            Step::RenderTemplate { when, .. } => when.as_deref(),
            Step::CreateFile { when, .. } => when.as_deref(),
            Step::Copy { when, .. } => when.as_deref(),
            Step::RunShell { when, .. } => when.as_deref(),
            Step::ApplyPatch { when, .. } => when.as_deref(),
        }
    }
}

/// A scaffold template manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Unique template identifier.
    pub id: String,

    /// Human-readable template name.
    pub name: String,

    /// Template description.
    #[serde(default)]
    pub description: Option<String>,

    /// Template version (SemVer).
    pub version: String,

    /// Target language (nullable for language-agnostic templates).
    #[serde(default)]
    pub language: Option<String>,

    /// Template inputs.
    #[serde(default)]
    pub inputs: Vec<Input>,

    /// Template steps.
    #[serde(default)]
    pub steps: Vec<Step>,

    /// CI provider.
    #[serde(default)]
    pub ci_provider: CiProvider,

    /// Network policy.
    #[serde(default)]
    pub network: NetworkPolicy,

    /// Test commands for `scaffold test`.
    #[serde(default)]
    pub tests: Vec<String>,

    /// Post-apply steps (run after main steps).
    #[serde(default)]
    pub post_apply: Vec<Step>,
}

impl Manifest {
    /// Load a manifest from a file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// Parse a manifest from a YAML string.
    pub fn parse(content: &str) -> Result<Self> {
        let manifest: Manifest = serde_yaml::from_str(content)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Validate the manifest.
    pub fn validate(&self) -> Result<()> {
        // Check required fields
        if self.id.is_empty() {
            return Err(ManifestError::ValidationError("id cannot be empty".into()));
        }
        if self.name.is_empty() {
            return Err(ManifestError::ValidationError(
                "name cannot be empty".into(),
            ));
        }
        if self.version.is_empty() {
            return Err(ManifestError::ValidationError(
                "version cannot be empty".into(),
            ));
        }

        // Validate SemVer format (basic check)
        if !is_valid_semver(&self.version) {
            return Err(ManifestError::ValidationError(format!(
                "invalid version format: {} (expected SemVer)",
                self.version
            )));
        }

        // Validate inputs
        for input in &self.inputs {
            if input.name.is_empty() {
                return Err(ManifestError::ValidationError(
                    "input name cannot be empty".into(),
                ));
            }
            if input.input_type == InputType::Enum && input.enum_values.is_empty() {
                return Err(ManifestError::ValidationError(format!(
                    "enum input '{}' must have enum_values",
                    input.name
                )));
            }
        }

        // Validate step conditions
        for step in self.steps.iter().chain(self.post_apply.iter()) {
            if let Some(cond) = step.condition() {
                validate_condition(cond)?;
            }
        }

        Ok(())
    }

    /// Get all required inputs.
    pub fn required_inputs(&self) -> Vec<&Input> {
        self.inputs.iter().filter(|i| i.required).collect()
    }

    /// Get all optional inputs.
    pub fn optional_inputs(&self) -> Vec<&Input> {
        self.inputs.iter().filter(|i| !i.required).collect()
    }
}

/// Basic SemVer validation.
fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| {
        // Allow pre-release suffixes like "0.1.0-alpha"
        let num_part = p.split('-').next().unwrap_or(p);
        num_part.parse::<u32>().is_ok()
    })
}

/// Validate a condition expression.
///
/// Supported syntax:
/// - `exists(var)` - check if variable exists
/// - `var == "value"` - equality check
/// - `var != "value"` - inequality check
fn validate_condition(condition: &str) -> Result<()> {
    let condition = condition.trim();

    // exists(var)
    if condition.starts_with("exists(") && condition.ends_with(')') {
        let var = &condition[7..condition.len() - 1];
        if var.is_empty() {
            return Err(ManifestError::ConditionError(
                "exists() requires a variable name".into(),
            ));
        }
        return Ok(());
    }

    // var == "value" or var != "value"
    if condition.contains("==") || condition.contains("!=") {
        // Basic syntax check - we'll do full evaluation at runtime
        return Ok(());
    }

    Err(ManifestError::ConditionError(format!(
        "unsupported condition syntax: {}",
        condition
    )))
}

/// Evaluate a condition against provided variables.
pub fn evaluate_condition(condition: &str, vars: &HashMap<String, serde_json::Value>) -> bool {
    let condition = condition.trim();

    // exists(var)
    if condition.starts_with("exists(") && condition.ends_with(')') {
        let var = &condition[7..condition.len() - 1];
        return vars.contains_key(var);
    }

    // var == "value"
    if let Some(pos) = condition.find("==") {
        let var = condition[..pos].trim();
        let value = condition[pos + 2..].trim().trim_matches('"');
        return match vars.get(var) {
            Some(serde_json::Value::String(s)) => s == value,
            Some(serde_json::Value::Bool(b)) => {
                (*b && value == "true") || (!*b && value == "false")
            }
            Some(serde_json::Value::Number(n)) => n.to_string() == value,
            _ => false,
        };
    }

    // var != "value"
    if let Some(pos) = condition.find("!=") {
        let var = condition[..pos].trim();
        let value = condition[pos + 2..].trim().trim_matches('"');
        return match vars.get(var) {
            Some(serde_json::Value::String(s)) => s != value,
            Some(serde_json::Value::Bool(b)) => {
                // b != (value == "true") - if b is true and value is "true", they're equal
                !(*b && value == "true" || !*b && value == "false")
            }
            Some(serde_json::Value::Number(n)) => n.to_string() != value,
            None => true, // Variable doesn't exist, so it's not equal
            _ => true,
        };
    }

    // Unknown condition - default to false
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_manifest() {
        let yaml = r#"
id: test-template
name: Test Template
version: "1.0.0"
"#;
        let manifest = Manifest::parse(yaml).unwrap();
        assert_eq!(manifest.id, "test-template");
        assert_eq!(manifest.name, "Test Template");
        assert_eq!(manifest.version, "1.0.0");
    }

    #[test]
    fn test_parse_full_manifest() {
        let yaml = r##"
id: rust-basic
name: Rust Basic Project
description: A basic Rust project template
version: "0.1.0"
language: rust
ci_provider: github_actions
network: deny

inputs:
  - name: project_name
    type: string
    prompt: "Project name?"
    required: true
  - name: use_async
    type: bool
    default: "false"
  - name: license
    type: enum
    enum_values:
      - MIT
      - Apache-2.0
      - GPL-3.0
    default: MIT

steps:
  - type: render_template
    src: Cargo.toml.hbs
    dest: Cargo.toml
  - type: create_file
    dest: README.md
    content: "# {{project_name}}"
  - type: run_shell
    cmd: cargo fmt
    when: exists(format_on_create)

tests:
  - cargo build
  - cargo test
"##;
        let manifest = Manifest::parse(yaml).unwrap();
        assert_eq!(manifest.id, "rust-basic");
        assert_eq!(manifest.language, Some("rust".into()));
        assert_eq!(manifest.ci_provider, CiProvider::GithubActions);
        assert_eq!(manifest.network, NetworkPolicy::Deny);
        assert_eq!(manifest.inputs.len(), 3);
        assert_eq!(manifest.steps.len(), 3);
        assert_eq!(manifest.tests.len(), 2);
    }

    #[test]
    fn test_invalid_semver() {
        let yaml = r#"
id: test
name: Test
version: "invalid"
"#;
        let result = Manifest::parse(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_exists_condition() {
        let mut vars = HashMap::new();
        vars.insert("foo".to_string(), serde_json::json!("bar"));

        assert!(evaluate_condition("exists(foo)", &vars));
        assert!(!evaluate_condition("exists(baz)", &vars));
    }

    #[test]
    fn test_evaluate_equality_condition() {
        let mut vars = HashMap::new();
        vars.insert("lang".to_string(), serde_json::json!("rust"));

        assert!(evaluate_condition("lang == \"rust\"", &vars));
        assert!(!evaluate_condition("lang == \"go\"", &vars));
        assert!(evaluate_condition("lang != \"go\"", &vars));
    }
}

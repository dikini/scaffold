//! Handlebars-based template rendering for scaffold.
//!
//! This crate provides template rendering capabilities using Handlebars,
//! with support for generating diffs and previews.

use handlebars::Handlebars;
use manifest::{Manifest, Step};
use serde::Serialize;
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

/// Errors that can occur during rendering.
#[derive(Debug, Error)]
pub enum RenderError {
    #[error("template error: {0}")]
    TemplateError(#[from] handlebars::TemplateError),

    #[error("render error: {0}")]
    RenderError(#[from] handlebars::RenderError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("template not found: {0}")]
    TemplateNotFound(String),

    #[error("invalid template path: {0}")]
    InvalidPath(String),
}

/// Result type for renderer operations.
pub type Result<T> = std::result::Result<T, RenderError>;

/// A rendered file with its content and metadata.
#[derive(Debug, Clone)]
pub struct RenderedFile {
    /// Relative path for the output file.
    pub path: PathBuf,
    /// Rendered content.
    pub content: String,
    /// Whether this file already exists in the target.
    pub exists: bool,
    /// Original content if file exists.
    pub original: Option<String>,
}

impl RenderedFile {
    /// Generate a unified diff if the file exists and has changes.
    pub fn diff(&self) -> Option<String> {
        let original = self.original.as_ref()?;
        if original == &self.content {
            return None;
        }

        let diff = TextDiff::from_lines(original, &self.content);
        let mut output = String::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            output.push_str(sign);
            output.push_str(change.value());
        }

        Some(output)
    }
}

/// Template renderer using Handlebars.
pub struct Renderer<'a> {
    handlebars: Handlebars<'a>,
    template_dir: PathBuf,
}

impl<'a> Renderer<'a> {
    /// Create a new renderer for a template directory.
    pub fn new<P: AsRef<Path>>(template_dir: P) -> Result<Self> {
        let handlebars = Handlebars::new();
        // Don't use strict mode - allow missing variables to render as empty

        Ok(Self {
            handlebars,
            template_dir: template_dir.as_ref().to_path_buf(),
        })
    }

    /// Render a single template string with variables.
    pub fn render_string<T: Serialize>(&self, template: &str, data: &T) -> Result<String> {
        Ok(self.handlebars.render_template(template, data)?)
    }

    /// Render a template file.
    pub fn render_file<T: Serialize>(&self, src: &str, data: &T) -> Result<String> {
        let path = self.template_dir.join("files").join(src);
        if !path.exists() {
            return Err(RenderError::TemplateNotFound(src.to_string()));
        }
        let template = std::fs::read_to_string(&path)?;
        self.render_string(&template, data)
    }

    /// Render all steps from a manifest.
    pub fn render_manifest<T: Serialize>(
        &self,
        manifest: &Manifest,
        data: &T,
        target_dir: &Path,
    ) -> Result<Vec<RenderedFile>> {
        let mut files = Vec::new();
        let vars: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(data).unwrap_or_default())
                .unwrap_or_default();

        for step in &manifest.steps {
            // Check condition
            if let Some(cond) = step.condition() {
                if !manifest::evaluate_condition(cond, &vars) {
                    continue;
                }
            }

            match step {
                Step::RenderTemplate { src, dest, .. } => {
                    let rendered_dest = self.render_string(dest, data)?;
                    let content = self.render_file(src, data)?;
                    let target_path = target_dir.join(&rendered_dest);

                    let (exists, original) = if target_path.exists() {
                        (true, Some(std::fs::read_to_string(&target_path)?))
                    } else {
                        (false, None)
                    };

                    files.push(RenderedFile {
                        path: PathBuf::from(rendered_dest),
                        content,
                        exists,
                        original,
                    });
                }
                Step::CreateFile { dest, content, .. } => {
                    let rendered_dest = self.render_string(dest, data)?;
                    let rendered_content = self.render_string(content, data)?;
                    let target_path = target_dir.join(&rendered_dest);

                    let (exists, original) = if target_path.exists() {
                        (true, Some(std::fs::read_to_string(&target_path)?))
                    } else {
                        (false, None)
                    };

                    files.push(RenderedFile {
                        path: PathBuf::from(rendered_dest),
                        content: rendered_content,
                        exists,
                        original,
                    });
                }
                Step::Copy { src, dest, .. } => {
                    let rendered_dest = self.render_string(dest, data)?;
                    let src_path = self.template_dir.join("files").join(src);

                    if src_path.is_file() {
                        let content = std::fs::read_to_string(&src_path)?;
                        let target_path = target_dir.join(&rendered_dest);

                        let (exists, original) = if target_path.exists() {
                            (true, Some(std::fs::read_to_string(&target_path)?))
                        } else {
                            (false, None)
                        };

                        files.push(RenderedFile {
                            path: PathBuf::from(rendered_dest),
                            content,
                            exists,
                            original,
                        });
                    } else if src_path.is_dir() {
                        // Copy directory recursively
                        for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
                            if entry.file_type().is_file() {
                                let rel_path = entry.path().strip_prefix(&src_path).unwrap();
                                let dest_path = PathBuf::from(&rendered_dest).join(rel_path);
                                let content = std::fs::read_to_string(entry.path())?;
                                let target_path = target_dir.join(&dest_path);

                                let (exists, original) = if target_path.exists() {
                                    (true, Some(std::fs::read_to_string(&target_path)?))
                                } else {
                                    (false, None)
                                };

                                files.push(RenderedFile {
                                    path: dest_path,
                                    content,
                                    exists,
                                    original,
                                });
                            }
                        }
                    }
                }
                // RunShell and ApplyPatch are handled by executor
                Step::RunShell { .. } | Step::ApplyPatch { .. } => {}
            }
        }

        Ok(files)
    }
}

/// Generate a preview of changes for display.
pub fn generate_preview(files: &[RenderedFile]) -> String {
    let mut output = String::new();

    for file in files {
        let status = if file.exists {
            if file.original.as_ref() == Some(&file.content) {
                "unchanged"
            } else {
                "modified"
            }
        } else {
            "created"
        };

        output.push_str(&format!("[{}] {}\n", status, file.path.display()));

        if let Some(diff) = file.diff() {
            output.push_str(&diff);
            output.push('\n');
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_string() {
        let renderer = Renderer {
            handlebars: Handlebars::new(),
            template_dir: PathBuf::new(),
        };

        let data = serde_json::json!({
            "name": "test-project",
            "version": "1.0.0"
        });

        let result = renderer
            .render_string("# {{name}} v{{version}}", &data)
            .unwrap();
        assert_eq!(result, "# test-project v1.0.0");
    }

    #[test]
    fn test_rendered_file_diff() {
        let file = RenderedFile {
            path: PathBuf::from("test.txt"),
            content: "line1\nline2\nline3\n".to_string(),
            exists: true,
            original: Some("line1\nold_line\nline3\n".to_string()),
        };

        let diff = file.diff().unwrap();
        assert!(diff.contains("-old_line"));
        assert!(diff.contains("+line2"));
    }
}

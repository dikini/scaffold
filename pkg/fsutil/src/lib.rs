//! Atomic file system operations for scaffold.
//!
//! This crate provides utilities for safely writing files to disk,
//! including temp-tree rendering and atomic directory moves.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use thiserror::Error;
use walkdir::WalkDir;

/// Errors that can occur during file system operations.
#[derive(Debug, Error)]
pub enum FsError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("path error: {0}")]
    PathError(String),

    #[error("directory not empty: {0}")]
    NotEmpty(String),

    #[error("file already exists: {0}")]
    AlreadyExists(String),
}

/// Result type for fsutil operations.
pub type Result<T> = std::result::Result<T, FsError>;

/// A staged file to be written.
#[derive(Debug, Clone)]
pub struct StagedFile {
    /// Relative path within the output directory.
    pub path: PathBuf,
    /// File content.
    pub content: String,
}

/// A temporary tree for staging file writes.
pub struct TempTree {
    temp_dir: TempDir,
    files: Vec<StagedFile>,
}

impl TempTree {
    /// Create a new temporary tree.
    pub fn new() -> Result<Self> {
        Ok(Self {
            temp_dir: TempDir::new()?,
            files: Vec::new(),
        })
    }

    /// Add a file to the staging area.
    pub fn add_file(&mut self, path: PathBuf, content: String) {
        self.files.push(StagedFile { path, content });
    }

    /// Get the path to the temp directory.
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Write all staged files to the temp directory.
    pub fn write_staged(&self) -> Result<()> {
        for file in &self.files {
            let full_path = self.temp_dir.path().join(&file.path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&full_path, &file.content)?;
        }
        Ok(())
    }

    /// Apply the temp tree to the target directory.
    ///
    /// This copies files from the temp directory to the target,
    /// creating directories as needed.
    pub fn apply_to(&self, target: &Path) -> Result<()> {
        self.write_staged()?;

        for file in &self.files {
            let src = self.temp_dir.path().join(&file.path);
            let dest = target.join(&file.path);

            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(&src, &dest)?;
        }

        Ok(())
    }

    /// Get list of staged files.
    pub fn staged_files(&self) -> &[StagedFile] {
        &self.files
    }
}

impl Default for TempTree {
    fn default() -> Self {
        Self::new().expect("failed to create temp directory")
    }
}

/// Copy a directory recursively.
pub fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    if !src.is_dir() {
        return Err(FsError::PathError(format!(
            "source is not a directory: {}",
            src.display()
        )));
    }

    fs::create_dir_all(dest)?;

    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let rel_path = entry.path().strip_prefix(src).unwrap();
        let target = dest.join(rel_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
    }

    Ok(())
}

/// Ensure a directory exists, creating it if necessary.
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    } else if !path.is_dir() {
        return Err(FsError::PathError(format!(
            "path exists but is not a directory: {}",
            path.display()
        )));
    }
    Ok(())
}

/// Check if a directory is empty.
pub fn is_dir_empty(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(true);
    }
    if !path.is_dir() {
        return Err(FsError::PathError(format!(
            "path is not a directory: {}",
            path.display()
        )));
    }
    Ok(fs::read_dir(path)?.next().is_none())
}

/// List all files in a directory recursively.
pub fn list_files(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_tree() {
        let mut tree = TempTree::new().unwrap();
        tree.add_file(PathBuf::from("test.txt"), "hello".to_string());
        tree.add_file(
            PathBuf::from("subdir/nested.txt"),
            "nested content".to_string(),
        );

        tree.write_staged().unwrap();

        assert!(tree.path().join("test.txt").exists());
        assert!(tree.path().join("subdir/nested.txt").exists());
    }

    #[test]
    fn test_is_dir_empty() {
        let temp = TempDir::new().unwrap();
        assert!(is_dir_empty(temp.path()).unwrap());

        fs::write(temp.path().join("file.txt"), "content").unwrap();
        assert!(!is_dir_empty(temp.path()).unwrap());
    }
}

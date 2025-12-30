//! Integration tests for scaffold CLI.

use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

/// Get the path to the scaffold binary.
fn scaffold_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove deps directory
    path.push("scaffold");
    path
}

/// Get the path to the templates directory.
fn templates_dir() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Go up from cmd/scaffold to workspace root
    manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("templates")
}

#[test]
fn test_list_templates() {
    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .arg("list-templates")
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust-basic"));
    assert!(stdout.contains("node-basic"));
    assert!(stdout.contains("ci-github-actions"));
}

#[test]
fn test_list_templates_json() {
    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .args(["list-templates", "--json"])
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse as JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["type"], "template_list");
    assert!(json["templates"].as_array().unwrap().len() >= 3);
}

#[test]
fn test_validate_rust_basic() {
    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .args(["validate", "--template", "rust-basic"])
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("valid"));
}

#[test]
fn test_validate_node_basic() {
    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .args(["validate", "--template", "node-basic"])
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
}

#[test]
fn test_validate_nonexistent_template() {
    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .args(["validate", "--template", "nonexistent"])
        .output()
        .expect("Failed to execute scaffold");

    assert!(!output.status.success());
}

#[test]
fn test_generate_dry_run() {
    let temp = assert_fs::TempDir::new().unwrap();
    let vars_file = temp.child("vars.yaml");
    vars_file
        .write_str("project_name: test_project\ndescription: A test project\n")
        .unwrap();

    let output_dir = temp.child("output");

    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .args([
            "generate",
            "--template",
            "rust-basic",
            "--out",
            output_dir.path().to_str().unwrap(),
            "--vars",
            vars_file.path().to_str().unwrap(),
            "--dry-run",
        ])
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Preview"));
    assert!(stdout.contains("Cargo.toml"));

    // Verify nothing was actually created
    assert!(!output_dir.path().join("Cargo.toml").exists());
}

#[test]
fn test_generate_apply() {
    let temp = assert_fs::TempDir::new().unwrap();
    let vars_file = temp.child("vars.yaml");
    vars_file
        .write_str("project_name: my_test_app\ndescription: Integration test project\nauthor: Test\nlicense: MIT\nuse_async: false\n")
        .unwrap();

    let output_dir = temp.child("output");

    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .args([
            "generate",
            "--template",
            "rust-basic",
            "--out",
            output_dir.path().to_str().unwrap(),
            "--vars",
            vars_file.path().to_str().unwrap(),
            "--apply",
        ])
        .output()
        .expect("Failed to execute scaffold");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("stdout: {}", stdout);
        eprintln!("stderr: {}", stderr);
    }

    assert!(output.status.success(), "Generate failed: {}", stderr);

    // Verify files were created
    output_dir
        .child("Cargo.toml")
        .assert(predicate::path::exists());
    output_dir
        .child("src/main.rs")
        .assert(predicate::path::exists());
    output_dir
        .child("src/lib.rs")
        .assert(predicate::path::exists());
    output_dir
        .child("README.md")
        .assert(predicate::path::exists());
    output_dir
        .child(".gitignore")
        .assert(predicate::path::exists());
}

#[test]
fn test_generate_json_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    let vars_file = temp.child("vars.yaml");
    vars_file.write_str("project_name: json_test\n").unwrap();

    let output_dir = temp.child("output");

    let output = Command::new(scaffold_bin())
        .env("SCAFFOLD_TEMPLATES", templates_dir())
        .args([
            "generate",
            "--template",
            "rust-basic",
            "--out",
            output_dir.path().to_str().unwrap(),
            "--vars",
            vars_file.path().to_str().unwrap(),
            "--dry-run",
            "--json",
        ])
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse as JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["type"], "generate");
    assert_eq!(json["dry_run"], true);
    assert!(json["files"].as_array().unwrap().len() > 0);
}

#[test]
fn test_help() {
    let output = Command::new(scaffold_bin())
        .arg("--help")
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("scaffold"));
    assert!(stdout.contains("list-templates"));
    assert!(stdout.contains("generate"));
    assert!(stdout.contains("validate"));
}

#[test]
fn test_version() {
    let output = Command::new(scaffold_bin())
        .arg("--version")
        .output()
        .expect("Failed to execute scaffold");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("scaffold"));
}

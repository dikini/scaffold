//! Scaffold CLI - Language-agnostic project scaffolding tool.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use executor::{Executor, ExecutorConfig};
use fsutil::TempTree;
use manifest::{Manifest, NetworkPolicy};
use opencode::{
    ChangeType, FileChange, GenerateResult, Output, Status, TemplateInfo, TestOutput,
    TestRunResult, ValidationResult,
};
use renderer::{generate_preview, RenderedFile, Renderer};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Scaffold - Language-agnostic project scaffolding tool
#[derive(Parser)]
#[command(name = "scaffold")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available templates
    ListTemplates {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Validate a template manifest
    Validate {
        /// Template ID or path
        #[arg(long)]
        template: String,

        /// Variables file (YAML)
        #[arg(long)]
        vars: Option<PathBuf>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Generate a project from a template
    Generate {
        /// Template ID or path
        #[arg(long)]
        template: String,

        /// Output directory
        #[arg(long)]
        out: PathBuf,

        /// Variables file (YAML)
        #[arg(long)]
        vars: Option<PathBuf>,

        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,

        /// Apply changes to disk
        #[arg(long)]
        apply: bool,

        /// Allow shell script execution
        #[arg(long)]
        allow_scripts: bool,

        /// Git commit after generation
        #[arg(long)]
        commit: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Initialize a new project interactively
    Init {
        /// Template ID (optional, will prompt if not provided)
        #[arg(long)]
        template: Option<String>,

        /// Run in interactive mode
        #[arg(long)]
        interactive: bool,

        /// Output directory (defaults to current directory)
        #[arg(long, default_value = ".")]
        out: PathBuf,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Run tests for a generated project
    Test {
        /// Target directory
        #[arg(long)]
        target: PathBuf,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListTemplates { json } => list_templates(json),
        Commands::Validate {
            template,
            vars,
            json,
        } => validate_template(&template, vars.as_deref(), json),
        Commands::Generate {
            template,
            out,
            vars,
            dry_run,
            apply,
            allow_scripts,
            commit,
            json,
        } => generate(
            &template,
            &out,
            vars.as_deref(),
            dry_run,
            apply,
            allow_scripts,
            commit,
            json,
        ),
        Commands::Init {
            template,
            interactive,
            out,
            json,
        } => init(template.as_deref(), interactive, &out, json),
        Commands::Test { target, json } => run_tests(&target, json),
    }
}

/// Get the templates directory path.
fn templates_dir() -> PathBuf {
    // First check for SCAFFOLD_TEMPLATES env var
    if let Ok(dir) = std::env::var("SCAFFOLD_TEMPLATES") {
        return PathBuf::from(dir);
    }

    // Then check relative to executable
    if let Ok(exe) = std::env::current_exe() {
        let exe_dir = exe.parent().unwrap_or(Path::new("."));
        let templates = exe_dir.join("templates");
        if templates.exists() {
            return templates;
        }
        // Also check parent (for dev builds)
        let parent_templates = exe_dir.parent().map(|p| p.join("templates"));
        if let Some(pt) = parent_templates {
            if pt.exists() {
                return pt;
            }
        }
    }

    // Default to templates in current working directory
    PathBuf::from("templates")
}

/// Find a template by ID or path.
fn find_template(template: &str) -> Result<PathBuf> {
    // If it's a path, use it directly
    let path = PathBuf::from(template);
    if path.exists() && path.join("scaffold.yaml").exists() {
        return Ok(path);
    }

    // Otherwise, look in templates directory
    let templates = templates_dir();
    let template_path = templates.join(template);
    if template_path.exists() && template_path.join("scaffold.yaml").exists() {
        return Ok(template_path);
    }

    anyhow::bail!("template not found: {}", template)
}

/// List available templates.
fn list_templates(json_output: bool) -> Result<()> {
    let templates_path = templates_dir();
    let mut templates = Vec::new();

    if templates_path.exists() {
        for entry in std::fs::read_dir(&templates_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("scaffold.yaml");
                if manifest_path.exists() {
                    if let Ok(manifest) = Manifest::from_file(&manifest_path) {
                        templates.push(TemplateInfo {
                            id: manifest.id,
                            name: manifest.name,
                            description: manifest.description,
                            version: manifest.version,
                            language: manifest.language,
                        });
                    }
                }
            }
        }
    }

    if json_output {
        let output = Output::TemplateList { templates };
        println!("{}", output.to_json()?);
    } else if templates.is_empty() {
        println!("No templates found in {}", templates_path.display());
    } else {
        println!("Available templates:\n");
        for t in &templates {
            println!("  {} ({})", t.id, t.version);
            println!("    {}", t.name);
            if let Some(desc) = &t.description {
                println!("    {}", desc);
            }
            if let Some(lang) = &t.language {
                println!("    Language: {}", lang);
            }
            println!();
        }
    }

    Ok(())
}

/// Validate a template.
fn validate_template(template: &str, vars_file: Option<&Path>, json_output: bool) -> Result<()> {
    let template_path = find_template(template)?;
    let manifest_path = template_path.join("scaffold.yaml");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Load and validate manifest
    let manifest = match Manifest::from_file(&manifest_path) {
        Ok(m) => Some(m),
        Err(e) => {
            errors.push(format!("manifest error: {}", e));
            None
        }
    };

    // If vars file provided, validate variables
    if let (Some(manifest), Some(vars_path)) = (&manifest, vars_file) {
        let vars: HashMap<String, serde_yaml::Value> =
            serde_yaml::from_str(&std::fs::read_to_string(vars_path)?)?;

        // Check required inputs
        for input in manifest.required_inputs() {
            if !vars.contains_key(&input.name) {
                errors.push(format!("missing required input: {}", input.name));
            }
        }

        // Warn about unknown variables
        for key in vars.keys() {
            if !manifest.inputs.iter().any(|i| &i.name == key) {
                warnings.push(format!("unknown variable: {}", key));
            }
        }
    }

    let valid = errors.is_empty();

    if json_output {
        let output = Output::Validation(ValidationResult {
            valid,
            errors,
            warnings,
        });
        println!("{}", output.to_json()?);
    } else if valid {
        println!("Template '{}' is valid", template);
        for w in &warnings {
            println!("  warning: {}", w);
        }
    } else {
        println!("Template '{}' validation failed:", template);
        for e in &errors {
            println!("  error: {}", e);
        }
        for w in &warnings {
            println!("  warning: {}", w);
        }
    }

    if valid {
        Ok(())
    } else {
        anyhow::bail!("validation failed")
    }
}

/// Generate a project from a template.
#[allow(clippy::too_many_arguments)]
fn generate(
    template: &str,
    out: &Path,
    vars_file: Option<&Path>,
    dry_run: bool,
    apply: bool,
    allow_scripts: bool,
    commit: bool,
    json_output: bool,
) -> Result<()> {
    let template_path = find_template(template)?;
    let manifest_path = template_path.join("scaffold.yaml");
    let manifest =
        Manifest::from_file(&manifest_path).context("failed to load template manifest")?;

    // Load variables
    let vars: serde_json::Value = if let Some(vars_path) = vars_file {
        let content = std::fs::read_to_string(vars_path)?;
        serde_yaml::from_str(&content)?
    } else {
        serde_json::json!({})
    };

    // Create renderer
    let renderer = Renderer::new(&template_path)?;

    // Render files
    let rendered_files = renderer.render_manifest(&manifest, &vars, out)?;

    // Convert to file changes
    let file_changes: Vec<FileChange> = rendered_files
        .iter()
        .map(|f| FileChange {
            path: f.path.clone(),
            change_type: if f.exists {
                if f.original.as_ref() == Some(&f.content) {
                    ChangeType::Unchanged
                } else {
                    ChangeType::Modified
                }
            } else {
                ChangeType::Created
            },
            diff: f.diff(),
        })
        .collect();

    if dry_run || !apply {
        // Preview mode
        if json_output {
            let output = Output::Generate(GenerateResult {
                status: Status::Success,
                output_dir: out.to_path_buf(),
                files: file_changes,
                dry_run: true,
                message: Some("Dry run - no changes applied".to_string()),
            });
            println!("{}", output.to_json()?);
        } else {
            println!("Preview of changes to {}:\n", out.display());
            println!("{}", generate_preview(&rendered_files));
            println!("\nUse --apply to write changes to disk.");
        }
        return Ok(());
    }

    // Apply mode
    apply_files(&rendered_files, out)?;

    // Run shell steps if allowed
    if allow_scripts {
        run_shell_steps(&manifest, out)?;
    }

    // Git commit if requested
    if commit {
        git_commit(out, &manifest.name)?;
    }

    if json_output {
        let output = Output::Generate(GenerateResult {
            status: Status::Success,
            output_dir: out.to_path_buf(),
            files: file_changes,
            dry_run: false,
            message: Some("Project generated successfully".to_string()),
        });
        println!("{}", output.to_json()?);
    } else {
        println!("Project generated successfully at {}", out.display());
        println!("\nFiles created:");
        for f in &rendered_files {
            let status = if f.exists { "updated" } else { "created" };
            println!("  [{}] {}", status, f.path.display());
        }
    }

    Ok(())
}

/// Apply rendered files to disk.
fn apply_files(files: &[RenderedFile], out: &Path) -> Result<()> {
    let mut tree = TempTree::new()?;

    for file in files {
        tree.add_file(file.path.clone(), file.content.clone());
    }

    tree.apply_to(out)?;
    Ok(())
}

/// Run shell steps from manifest.
fn run_shell_steps(manifest: &Manifest, workdir: &Path) -> Result<()> {
    let executor = Executor::new(ExecutorConfig {
        network_policy: manifest.network,
        allow_scripts: true,
        capture_output: true,
    });

    let vars: HashMap<String, serde_json::Value> = HashMap::new();

    for step in &manifest.steps {
        if let Some(cond) = step.condition() {
            if !manifest::evaluate_condition(cond, &vars) {
                continue;
            }
        }

        if let manifest::Step::RunShell {
            cmd,
            workdir: step_workdir,
            ..
        } = step
        {
            let dir = step_workdir
                .as_ref()
                .map(|w| workdir.join(w))
                .unwrap_or_else(|| workdir.to_path_buf());

            println!("Running: {}", cmd);
            executor.execute(cmd, Some(&dir))?;
        }
    }

    Ok(())
}

/// Git commit the generated project.
fn git_commit(dir: &Path, name: &str) -> Result<()> {
    let executor = Executor::new(ExecutorConfig {
        network_policy: NetworkPolicy::Deny,
        allow_scripts: true,
        capture_output: true,
    });

    // Check if .git exists
    if !dir.join(".git").exists() {
        executor.execute("git init", Some(dir))?;
    }

    executor.execute("git add .", Some(dir))?;
    executor.execute(
        &format!("git commit -m \"feat: scaffold {} project\"", name),
        Some(dir),
    )?;

    Ok(())
}

/// Initialize a new project.
fn init(template: Option<&str>, _interactive: bool, out: &Path, json_output: bool) -> Result<()> {
    // If no template specified, list available templates
    let template_id = match template {
        Some(t) => t.to_string(),
        None => {
            if json_output {
                anyhow::bail!("--template is required in non-interactive mode");
            }
            // For now, just fail - interactive mode would prompt
            anyhow::bail!("--template is required (interactive mode not yet implemented)");
        }
    };

    // For now, init is an alias for generate with --apply
    generate(
        &template_id,
        out,
        None,
        false,
        true,
        false,
        false,
        json_output,
    )
}

/// Run tests for a generated project.
fn run_tests(target: &Path, json_output: bool) -> Result<()> {
    // Look for scaffold.yaml in target to find test commands
    let manifest_path = target.join("scaffold.yaml");

    let tests = if manifest_path.exists() {
        let manifest = Manifest::from_file(&manifest_path)?;
        manifest.tests
    } else {
        // Try common test commands
        vec![
            "cargo test".to_string(),
            "npm test".to_string(),
            "go test ./...".to_string(),
        ]
    };

    if tests.is_empty() {
        if json_output {
            let output = Output::TestRun(TestRunResult {
                status: Status::Warning,
                tests: vec![],
                total: 0,
                passed: 0,
                failed: 0,
            });
            println!("{}", output.to_json()?);
        } else {
            println!("No tests defined for this project.");
        }
        return Ok(());
    }

    let executor = Executor::new(ExecutorConfig {
        network_policy: NetworkPolicy::Allow,
        allow_scripts: true,
        capture_output: true,
    });

    let results = executor.run_tests(&tests, target)?;

    let test_outputs: Vec<TestOutput> = results
        .iter()
        .map(|r| TestOutput {
            command: r.command.clone(),
            passed: r.success,
            output: if r.success {
                Some(r.stdout.clone())
            } else {
                None
            },
            error: if !r.success {
                Some(r.stderr.clone())
            } else {
                None
            },
        })
        .collect();

    let passed = results.iter().filter(|r| r.success).count();
    let failed = results.len() - passed;

    if json_output {
        let output = Output::TestRun(TestRunResult {
            status: if failed == 0 {
                Status::Success
            } else {
                Status::Error
            },
            tests: test_outputs,
            total: results.len(),
            passed,
            failed,
        });
        println!("{}", output.to_json()?);
    } else {
        println!("Test results:\n");
        for r in &results {
            let status = if r.success { "PASS" } else { "FAIL" };
            println!("[{}] {}", status, r.command);
            if !r.success {
                println!("  {}", r.stderr);
            }
        }
        println!(
            "\nTotal: {} | Passed: {} | Failed: {}",
            results.len(),
            passed,
            failed
        );
    }

    if failed > 0 {
        anyhow::bail!("{} test(s) failed", failed);
    }

    Ok(())
}

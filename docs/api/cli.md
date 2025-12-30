# CLI Reference

Complete reference for all Scaffold CLI commands and options.

## 📋 Command Overview

```
scaffold <COMMAND> [OPTIONS]

Commands:
  list-templates    List available templates
  validate         Validate a template manifest
  generate         Generate a project from a template
  init             Initialize a new project interactively
  test             Run tests for a generated project
  help            Print help or help for subcommand
```

## 🗃️ Global Options

Available for all commands:

### `--help` / `-h`
Show help information.

```bash
scaffold --help
scaffold generate --help
```

### `--version` / `-V`
Show version information.

```bash
scaffold --version
# Output: scaffold 0.1.0
```

## 📚 Commands

### `scaffold list-templates`

List all available templates.

#### Syntax
```bash
scaffold list-templates [--json]
```

#### Options
- `--json`: Output as JSON (for opencode integration)

#### Examples
```bash
# List templates
scaffold list-templates

# JSON output
scaffold list-templates --json
```

#### Output Format
**Default (text):**
```
Available templates:

  rust-basic (0.1.0)
    Rust Basic Project
    A basic Rust project with CI, linting, and tests
    Language: rust

  node-basic (0.1.0)
    Node.js/TypeScript Basic Project
    A basic Node.js project with TypeScript, CI, linting, and tests
    Language: typescript
```

**JSON:**
```json
{
  "type": "template_list",
  "templates": [
    {
      "id": "rust-basic",
      "name": "Rust Basic Project",
      "description": "A basic Rust project with CI, linting, and tests",
      "version": "0.1.0",
      "language": "rust"
    }
  ]
}
```

### `scaffold validate`

Validate a template manifest without generating a project.

#### Syntax
```bash
scaffold validate --template <TEMPLATE> [--vars <VARS>] [--json]
```

#### Required Options
- `--template <TEMPLATE>`: Template ID or path to template directory

#### Optional Options
- `--vars <VARS>`: Path to YAML file with variables for validation
- `--json`: Output validation result as JSON

#### Examples
```bash
# Validate template by ID
scaffold validate --template rust-basic

# Validate with variables
scaffold validate --template rust-basic --vars my-vars.yaml

# Validate local template
scaffold validate --template ./my-template

# JSON output
scaffold validate --template rust-basic --json
```

#### Exit Codes
- `0`: Validation successful
- `1`: Validation failed or error

#### Output Format
**Default (text):**
```
Template 'rust-basic' is valid
  warning: unknown variable: debug_mode
```

**JSON:**
```json
{
  "type": "validation",
  "valid": false,
  "errors": ["Missing required field: id"],
  "warnings": ["Unknown variable: debug_mode"]
}
```

### `scaffold generate`

Generate a project from a template.

#### Syntax
```bash
scaffold generate --template <TEMPLATE> --out <DIR> [OPTIONS]
```

#### Required Options
- `--template <TEMPLATE>`: Template ID or path to template directory
- `--out <DIR>`: Output directory for generated project

#### Optional Options
- `--vars <VARS>`: Path to YAML file with variables
- `--dry-run`: Preview changes without applying them
- `--apply`: Apply changes to disk
- `--allow-scripts`: Allow shell script execution
- `--commit`: Git commit after generation
- `--json`: Output as JSON (for opencode integration)

#### Examples
```bash
# Basic generation
scaffold generate --template rust-basic --out my-project --apply

# With variables
scaffold generate --template rust-basic --out my-project --vars vars.yaml --apply

# Dry run preview
scaffold generate --template rust-basic --out my-project --vars vars.yaml --dry-run

# Allow scripts (for npm install, etc.)
scaffold generate --template node-basic --out my-app --apply --allow-scripts

# Auto-commit
scaffold generate --template rust-basic --out my-project --apply --commit

# JSON output
scaffold generate --template rust-basic --out my-project --apply --json
```

#### Workflow Patterns

**Safe Generation (recommended):**
```bash
# 1. Preview changes
scaffold generate --template rust-basic --out my-project --dry-run

# 2. Review output, then apply
scaffold generate --template rust-basic --out my-project --apply
```

**Quick Generation:**
```bash
# Direct generation
scaffold generate --template rust-basic --out my-project --apply
```

**Trusted Template Generation:**
```bash
# Allow scripts and commit
scaffold generate --template node-basic --out my-app --vars vars.yaml --apply --allow-scripts --commit
```

#### Output Format
**Default (text):**
```
Project generated successfully at /path/to/my-project

Files created:
  [created] Cargo.toml
  [created] src/main.rs
  [created] README.md
```

**JSON:**
```json
{
  "type": "generate",
  "status": "success",
  "output_dir": "/path/to/my-project",
  "files": [
    {
      "path": "Cargo.toml",
      "change_type": "created",
      "diff": null
    }
  ],
  "dry_run": false,
  "message": "Project generated successfully"
}
```

### `scaffold init`

Initialize a new project interactively.

#### Syntax
```bash
scaffold init [--template <TEMPLATE>] [--interactive] [--out <DIR>] [--json]
```

#### Optional Options
- `--template <TEMPLATE>`: Skip template selection, use specified template
- `--interactive`: Force interactive mode (default when no template specified)
- `--out <DIR>`: Output directory (default: current directory)
- `--json`: Output as JSON

#### Examples
```bash
# Interactive mode (default)
scaffold init

# Specify template
scaffold init --template rust-basic

# Custom output directory
scaffold init --out new-project

# JSON output
scaffold init --json
```

#### Interactive Workflow
1. **Template Selection**: Choose from available templates
2. **Variable Input**: Prompts for each required input
3. **Configuration**: Confirm network and script options
4. **Generation**: Create project with preview
5. **Confirmation**: Apply changes to disk

### `scaffold test`

Run tests for a generated project.

#### Syntax
```bash
scaffold test --target <DIR> [--json]
```

#### Required Options
- `--target <DIR>`: Directory containing generated project to test

#### Optional Options
- `--json`: Output test results as JSON

#### Examples
```bash
# Test generated project
scaffold test --target my-rust-project

# JSON output
scaffold test --target my-rust-project --json
```

#### Test Discovery

Scaffold looks for tests in this order:
1. **scaffold.yaml** in target directory
2. **Language-specific defaults**:
   - Rust: `cargo build`, `cargo test`
   - Node.js: `npm run build`, `npm test`
   - Go: `go build`, `go test`
   - Python: `python -m build`, `python -m pytest`

#### Output Format
**Default (text):**
```
Test results:

Test results:
  [PASS] cargo build
  [PASS] cargo test
  [PASS] cargo clippy -- -D warnings

Total: 3 | Passed: 3 | Failed: 0
```

**JSON:**
```json
{
  "type": "test_run",
  "status": "success",
  "tests": [
    {
      "command": "cargo build",
      "passed": true,
      "output": "   Compiling ...\n    Finished dev profile",
      "error": null
    }
  ],
  "total": 3,
  "passed": 3,
  "failed": 0
}
```

## 🔧 Environment Variables

Configure Scaffold behavior globally:

### `SCAFFOLD_TEMPLATES`
Custom templates directory.

```bash
export SCAFFOLD_TEMPLATES="/path/to/my/templates"
scaffold list-templates  # Uses custom templates
```

### `SCAFFOLD_NETWORK_POLICY`
Default network policy when not specified in template.

```bash
export SCAFFOLD_NETWORK_POLICY="deny"
```

Values: `allow`, `deny`, `ask`

### `NO_COLOR`
Disable colored output.

```bash
export NO_COLOR=1
scaffold generate --template rust-basic --out my-project --apply
```

## 📊 Exit Codes

| Code | Meaning | Command |
|-------|---------|----------|
| 0 | Success | All |
| 1 | General error | All |
| 2 | Template not found | generate, validate, init |
| 3 | Validation failed | generate, validate |
| 4 | File system error | generate, init |
| 5 | Network policy violation | generate |
| 6 | Script execution denied | generate |
| 7 | Test execution failed | test |

## 🔍 Common Error Messages

### Template Not Found
```
Error: template not found: unknown-template
```

**Solution**: Check available templates with `scaffold list-templates`

### Validation Failed
```
Error: validation failed
```

**Solution**: Fix template manifest or variables

### Network Policy Violation
```
Error: command blocked by network policy: npm install
```

**Solution**: Use `--allow-scripts` or change network policy

### Script Execution Denied
```
Error: script execution not allowed (use --allow-scripts)
```

**Solution**: Add `--allow-scripts` flag if you trust the template

### File Already Exists
```
Error: IO error: File already exists (os error 17)
```

**Solution**: Choose different output directory or clean existing files

## 🎯 Pro Tips

### Efficiency
1. **Use dry-run** first to preview changes
2. **Reuse variables files** for consistent projects
3. **Set environment variables** for custom templates directory
4. **Use JSON output** for automation scripts

### Safety
1. **Review templates** before applying with `--allow-scripts`
2. **Use network: deny** for security-sensitive environments
3. **Validate templates** before using in production
4. **Check variable files** for sensitive data

### Automation
```bash
# Script for multiple projects
#!/bin/bash

projects=("web" "api" "cli")
for proj in "${projects[@]}"; do
  scaffold generate \
    --template rust-basic \
    --out "$proj" \
    --vars "${proj}.yaml" \
    --apply \
    --commit
done
```

## 🔗 See Also

- [Getting Started Guide](../getting-started.md) - Learn the basics
- [Template Authoring](../templates.md) - Create templates
- [Manifest Schema](manifest.md) - Template specification
- [Configuration Guide](../configuration.md) - Environment variables
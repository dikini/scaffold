# Manifest Schema Reference

Complete specification for `scaffold.yaml` template manifests.

## 🏗️ Schema Overview

A scaffold.yaml file defines a template's metadata, inputs, and generation steps.

```yaml
# Top-level fields
id: string                    # Required
name: string                  # Required
description: string             # Optional
version: string                # Required (SemVer)
language: string|null           # Optional
ci_provider: enum            # Optional
network: enum                 # Optional

# Input definitions
inputs: array                  # Optional

# Generation steps
steps: array                  # Optional

# Test commands
tests: array                  # Optional

# Post-apply steps
post_apply: array             # Optional
```

## 📝 Top-level Fields

### id
**Type**: `string` | Required

Unique identifier for the template. Used with `--template` option.

```yaml
id: rust-basic
id: node-web-app
id: my-custom-template
```

**Constraints**:
- Must be unique across all templates
- Use lowercase, hyphens, and underscores
- Avoid spaces and special characters

### name
**Type**: `string` | Required

Human-readable name displayed in listings and help.

```yaml
name: Rust Basic Project
name: Node.js Web Application
name: My Custom Template
```

### description
**Type**: `string` | Optional

Short description of what the template does.

```yaml
description: A basic Rust project with CI, linting, and tests
description: Full-stack Node.js app with Express and TypeScript
```

### version
**Type**: `string` | Required

Semantic version following SemVer specification.

```yaml
version: "1.0.0"
version: "0.2.1"
version: "2.0.0-alpha.1"
```

**SemVer Format**: `MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]`

### language
**Type**: `string` | `null` | Optional | Default: `null`

Target programming language. Use `null` for language-agnostic templates.

```yaml
language: rust
language: typescript
language: python
language: null          # Language-agnostic
```

**Common Values**:
- `rust`, `typescript`, `javascript`, `python`, `go`, `java`, `c++`
- `null` for multi-language or tooling templates

### ci_provider
**Type**: `enum` | Optional | Default: `github_actions`

CI/CD platform to configure in generated projects.

```yaml
ci_provider: github_actions
ci_provider: gitlab_ci
ci_provider: azure_pipelines
ci_provider: none
```

**Values**:
- `github_actions`: GitHub Actions workflows
- `gitlab_ci`: GitLab CI configuration
- `azure_pipelines`: Azure DevOps pipelines
- `none`: No CI configuration

### network
**Type**: `enum` | Optional | Default: `allow`

Default network policy for template scripts.

```yaml
network: allow           # No restrictions
network: deny            # Block network calls
network: ask             # Prompt for network access
```

## 🔧 Input Definitions

### inputs Array
Array of input objects defining template variables.

```yaml
inputs:
  - name: project_name
    type: string
    prompt: "Project name?"
    required: true
    default: "my-project"
  
  - name: license
    type: enum
    enum_values:
      - MIT
      - Apache-2.0
      - GPL-3.0
    default: MIT
```

### Input Fields

#### name
**Type**: `string` | Required

Variable name used in templates. Must be a valid identifier.

```yaml
name: project_name
name: api_port
name: author_email
```

#### type
**Type**: `enum` | Optional | Default: `string`

Data type for the input.

```yaml
type: string
type: bool
type: int
type: enum
```

**Types**:
- `string`: Text value
- `bool`: Boolean (true/false)
- `int`: Integer value
- `enum`: Selection from predefined values

#### prompt
**Type**: `string` | Optional

User-facing prompt for interactive mode.

```yaml
prompt: "Project name?"
prompt: "Enable async mode?"
prompt: "Choose license:"
```

#### default
**Type**: `string` | Optional

Default value when user doesn't provide one.

```yaml
default: "my-project"
default: "false"
default: "8080"
default: "MIT"
```

**Note**: Always provided as string, parsed according to type.

#### required
**Type**: `bool` | Optional | Default: `false`

Whether this input must be provided.

```yaml
required: true
required: false
```

#### enum_values
**Type**: `array` | Required when `type: enum`

List of allowed values for enum inputs.

```yaml
type: enum
enum_values:
  - MIT
  - Apache-2.0
  - GPL-3.0
  - none
```

## ⚡ Step Definitions

### steps Array
Ordered list of generation steps. Executed in sequence.

```yaml
steps:
  - type: render_template
    src: Cargo.toml.hbs
    dest: Cargo.toml
    when: "language == 'rust'"
  
  - type: run_shell
    cmd: "cargo fmt"
    when: "exists(format_code)"
```

### Step Types

#### render_template
Process a Handlebars template file.

```yaml
type: render_template
src: string              # Required - source template path
dest: string             # Required - destination path
when: string             # Optional - condition expression
```

**Fields**:
- `src`: Source file path relative to `files/` directory
- `dest`: Destination path relative to output directory
- `when`: Condition expression (see [Conditions](#conditions))

#### create_file
Create a file with inline Handlebars content.

```yaml
type: create_file
dest: string             # Required - destination path
content: string          # Required - Handlebars content
when: string             # Optional - condition expression
```

#### copy
Copy files or directories as-is.

```yaml
type: copy
src: string              # Required - source path
dest: string             # Required - destination path
when: string             # Optional - condition expression
```

**Behavior**:
- Files: Direct copy
- Directories: Recursive copy preserving structure

#### run_shell
Execute shell commands in the output directory.

```yaml
type: run_shell
cmd: string              # Required - command to execute
workdir: string          # Optional - working directory
when: string             # Optional - condition expression
```

**Security**: Requires `--allow-scripts` flag unless network policy is `allow`.

#### apply_patch
Apply a patch file to the output directory.

```yaml
type: apply_patch
patch: string            # Required - patch file path
when: string             # Optional - condition expression
```

### post_apply Array
Steps executed after main generation steps.

```yaml
post_apply:
  - type: run_shell
    cmd: "git init"
    when: "initialize_git"
  
  - type: create_file
    dest: NEXT.md
    content: "Next steps for {{project_name}}"
```

**Use Cases**:
- Git initialization
- Post-generation setup
- User notifications
- Cleanup operations

## 🧪 Test Definitions

### tests Array
List of test commands for the generated project.

```yaml
tests:
  - "cargo build"
  - "cargo test"
  - "cargo clippy -- -D warnings"
  - "npm run build"
  - "npm test"
  - "python -m pytest"
```

**Usage**: Executed by `scaffold test --target <directory>`

## 🎯 Conditions

Control conditional execution of steps using expressions.

### Syntax
```yaml
when: "exists(var_name)"           # Check if variable exists
when: "var == 'value'"          # Equality check
when: "var != 'value'"          # Inequality check
```

### exists()
Check if a variable is present and not empty.

```yaml
when: "exists(author)"           # True if author is provided
when: "exists(debug_mode)"       # True if debug_mode is set
```

### Equality
```yaml
when: "language == 'rust'"      # True if language equals "rust"
when: "use_async == 'true'"     # True if use_async is true
```

### Inequality
```yaml
when: "license != 'none'"       # True if license is not "none"
when: "use_async != 'false'"    # True if use_async is not false
```

### Complex Conditions
Combine multiple conditions in Handlebars templates:

```handlebars
{{#if (and (eq language "rust") (eq license "MIT"))}}
[package]
license = "MIT"
{{/if}}
```

## 📚 Complete Example

```yaml
# template-name/scaffold.yaml
id: rust-web-app
name: Rust Web Application
description: A Rust web app with Axum and CI
version: "1.0.0"
language: rust
ci_provider: github_actions
network: deny

inputs:
  - name: project_name
    type: string
    prompt: "Project name?"
    required: true
    default: "my-web-app"
  
  - name: description
    type: string
    prompt: "Project description?"
    default: "A Rust web application"
  
  - name: author
    type: string
    prompt: "Author name?"
    default: ""
  
  - name: license
    type: enum
    enum_values:
      - MIT
      - Apache-2.0
      - GPL-3.0
      - none
    default: MIT
  
  - name: use_async
    type: bool
    prompt: "Use async/await?"
    default: "false"
  
  - name: enable_tracing
    type: bool
    prompt: "Enable tracing?"
    default: "true"

steps:
  - type: render_template
    src: Cargo.toml.hbs
    dest: Cargo.toml
  
  - type: render_template
    src: src/main.rs.hbs
    dest: src/main.rs
  
  - type: render_template
    dest: README.md
    content: |
      # {{project_name}}
      
      {{description}}
      
      ## Getting Started
      
      ```bash
      cargo build
      cargo run
      ```
  
  - type: copy
    src: .github
    dest: .github
  
  - type: run_shell
    cmd: "cargo fmt"
    when: "exists(format_code)"
  
  - type: run_shell
    cmd: "cargo clippy -- -D warnings"
    when: "exists(run_lint)"

tests:
  - "cargo build"
  - "cargo test"
  - "cargo clippy -- -D warnings"

post_apply:
  - type: run_shell
    cmd: "git init && git add . && git commit -m 'feat: scaffold {{project_name}} project'"
    when: "initialize_git"
```

## ✅ Validation

Scaffold validates manifests for:

### Required Fields
- `id` must be non-empty
- `name` must be non-empty
- `version` must be valid SemVer

### Input Validation
- Input `name` must be unique
- `enum` inputs must have `enum_values`
- Variable names must be valid identifiers

### Condition Validation
- Valid syntax for `when` expressions
- Proper escaping in Handlebars templates

### Step Validation
- All required fields present
- Valid `type` values
- Proper file paths in `src` and `dest`

## 🔗 See Also

- [Template Authoring Guide](../templates.md) - Practical template creation
- [CLI Reference](cli.md) - Command usage
- [Examples](../examples/) - Real-world templates
- [Getting Started](../getting-started.md) - Learn Scaffold basics
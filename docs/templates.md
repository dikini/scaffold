# Templates

This guide covers everything about Scaffold templates: using existing ones, creating your own, and advanced template features.

## 🏗️ Template Structure

Every template follows this structure:

```
template-name/
├── scaffold.yaml          # Template manifest
└── files/                # Template files and directories
    ├── Cargo.toml.hbs     # Handlebars templates
    ├── src/
    │   ├── main.rs.hbs
    │   └── lib.rs.hbs
    └── .github/
        └── workflows/
            └── ci.yml.hbs
```

### scaffold.yaml

The manifest defines template metadata, inputs, and generation steps.

### files/ Directory

Contains all files that will be generated:
- **Handlebars files** (`.hbs` extension) - processed with variables
- **Regular files** - copied as-is
- **Directory structure** - preserved during generation

## 📝 Template Manifest

### Basic Structure

```yaml
id: template-id                    # Unique identifier
name: Human Readable Name          # Display name
description: What this template does # Short description
version: "1.0.0"                  # SemVer version
language: rust                      # Target language (optional)

inputs:                              # Template variables
  - name: project_name
    type: string
    prompt: "Project name?"
    required: true

steps:                                # Generation steps
  - type: render_template
    src: Cargo.toml.hbs
    dest: Cargo.toml

tests:                                 # Test commands
  - cargo build
  - cargo test
```

### Complete Manifest Reference

See [Manifest Schema](../api/manifest.md) for detailed specification.

## 🔧 Input Types

### string
```yaml
- name: project_name
  type: string
  prompt: "Project name?"
  default: "my-project"
  required: true
```

### bool
```yaml
- name: use_async
  type: bool
  prompt: "Use async?"
  default: "false"
```

### int
```yaml
- name: port
  type: int
  prompt: "Server port?"
  default: "8080"
```

### enum
```yaml
- name: license
  type: enum
  enum_values:
    - MIT
    - Apache-2.0
    - GPL-3.0
  default: MIT
```

## ⚡ Step Types

### render_template
Process a Handlebars template:

```yaml
- type: render_template
  src: Cargo.toml.hbs          # Source template
  dest: Cargo.toml              # Destination path
  when: "language == 'rust'"   # Optional condition
```

### create_file
Create a file with inline content:

```yaml
- type: create_file
  dest: README.md
  content: "# {{project_name}}"   # Handlebars content
  when: "license != 'none'"
```

### copy
Copy files or directories as-is:

```yaml
- type: copy
  src: .github                  # Source in template
  dest: .github                 # Destination in project
```

### run_shell
Execute shell commands:

```yaml
- type: run_shell
  cmd: cargo fmt                 # Command to run
  workdir: .                    # Working directory (optional)
  when: "exists(format_code)"      # Condition
```

### apply_patch
Apply a patch file:

```yaml
- type: apply_patch
  patch: fixes.patch              # Patch file
  when: "needs_fixes"           # Condition
```

## 🎯 Conditions

Conditions control when steps execute using simple expressions:

### exists()
Check if a variable is present:

```yaml
- type: run_shell
  cmd: cargo fmt
  when: "exists(format_code)"
```

### Equality

```yaml
- type: render_template
  src: async-main.rs.hbs
  dest: src/main.rs
  when: "use_async == 'true'"
```

### Inequality

```yaml
- type: render_template
  src: sync-main.rs.hbs
  dest: src/main.rs
  when: "use_async != 'true'"
```

### Complex Conditions

Combine multiple conditions:

```yaml
# Template file
{{#if (eq language "rust" and (eq license "MIT"))}}
[package]
license = "MIT"
{{else}}
[package]
license = "{{license}}"
{{/if}}
```

## 🔧 Handlebars Usage

### Basic Variables
```handlebars
# Simple variable
Project: {{project_name}}

# Object property
Author: {{author.name}}

# Array iteration
{{#each dependencies}}
{{this}}
{{/each}}
```

### Conditionals
```handlebars
{{#if use_async}}
#[tokio::main]
async fn main() {
{{else}}
fn main() {
{{/if}}
```

### Helpers
Scaffold provides additional Handlebars helpers:

#### eq
```handlebars
{{#if (eq license "MIT")}}
License: MIT
{{/if}}
```

#### unless
```handlebars
{{#unless (eq license "none")}}
License: {{license}}
{{/unless}}
```

## 🔒 Network Policies

Control network access during template generation:

### deny (Default for Security)
```yaml
network: deny
```
Blocks commands like: `curl`, `npm install`, `pip install`, `cargo install`

### allow
```yaml
network: allow
```
No network restrictions.

### ask
```yaml
network: ask
```
Prompts user before allowing network commands.

## 🛡️ Script Execution

Control whether scripts can run:

### Security by Default
```yaml
# Default behavior - scripts blocked unless explicitly allowed
steps:
  - type: run_shell
    cmd: npm install    # User must use --allow-scripts
```

### Safe Script Patterns
```yaml
steps:
  - type: run_shell
    cmd: cargo fmt        # Safe - local operation
  - type: run_shell  
    cmd: cargo clippy     # Safe - local operation
  - type: run_shell
    cmd: "git init"       # Safe - local operation
```

### Network Operations
```yaml
steps:
  - type: run_shell
    cmd: npm install      # Requires --allow-scripts
  - type: run_shell
    cmd: cargo install    # Requires --allow-scripts
  - type: run_shell
    cmd: curl https://...  # Requires --allow-scripts
```

## 📚 Best Practices

### Security
1. **Default to `network: deny`** for security-focused templates
2. **Use specific commands** instead of generic ones
3. **Validate inputs** before using in shell commands
4. **Document script requirements** clearly

### Usability
1. **Provide sensible defaults** for all inputs
2. **Use descriptive prompts** and clear input names
3. **Group related files** in logical directory structures
4. **Handle edge cases** gracefully

### Maintainability
1. **Version your templates** with SemVer
2. **Document special requirements** in description
3. **Test with various inputs** before release
4. **Keep templates modular** for reuse

## 🎨 Template Authoring

### Step 1: Create Template Directory

```bash
mkdir templates/my-template
cd templates/my-template
```

### Step 2: Write Manifest

```yaml
# templates/my-template/scaffold.yaml
id: my-template
name: My Custom Template
description: A custom project template
version: "1.0.0"
language: rust

inputs:
  - name: project_name
    type: string
    prompt: "Project name?"
    required: true

steps:
  - type: render_template
    src: main.rs.hbs
    dest: src/main.rs
```

### Step 3: Add Template Files

```rust
// templates/my-template/files/src/main.rs.hbs
fn main() {
    println!("Hello from {{project_name}}!");
}
```

### Step 4: Test Template

```bash
# Validate manifest
scaffold validate --template my-template

# Test generation
scaffold generate \
  --template my-template \
  --out test-project \
  --vars project_name=test-app \
  --dry-run
```

### Step 5: Document Usage

Create README in template:

```
templates/my-template/README.md
# My Template

Usage:
scaffold generate --template my-template --out your-project

Variables:
- project_name: Name of the project (required)
```

## 🔄 Template Versioning

Use Semantic Versioning:

```yaml
version: "1.0.0"    # Major.Minor.Patch

# Breaking changes: 2.0.0
# New features: 1.1.0  
# Bug fixes: 1.0.1
```

Update changelog in template README:

```markdown
## Changelog

### 1.1.0 (2024-01-15)
- Add TypeScript support
- Improve error handling

### 1.0.0 (2024-01-01)
- Initial release
```

## 📦 Sharing Templates

### Local Templates
```bash
# Use template in local directory
scaffold generate --template ./my-custom-template --out project
```

### Repository Templates
Structure for template repository:

```
my-templates/
├── README.md                 # Overview of all templates
├── rust-web/              # Rust web app template
│   ├── scaffold.yaml
│   └── files/
├── node-api/               # Node.js API template
│   ├── scaffold.yaml
│   └── files/
└── shared/                 # Shared components
    ├── common-workflows/
    └── snippets/
```

### Community Templates
1. **Submit to Scaffold registry** (when available)
2. **Share via GitHub** with documentation
3. **Include examples** and usage instructions
4. **Test thoroughly** before release

## 🔧 Advanced Features

### Post-apply Steps
Run after main steps:

```yaml
post_apply:
  - type: run_shell
    cmd: "git init && git add . && git commit -m 'Initial commit'"
    when: "initialize_git"
```

### Conditional Files
Create files only when conditions are met:

```handlebars
{{#if (eq license "MIT")}}
[package]
license = "MIT"
{{/if}}

{{#unless (eq author "")}}
[package]
authors = ["{{author}}"]
{{/unless}}
```

### Variable Validation
Validate inputs in templates:

```handlebars
{{#if (eq project_name "")}}
Error: project_name is required
{{/if}}
```

## 🎯 Next Steps

Now that you understand templates:

1. **Read Examples** - See [real-world templates](../examples/)
2. **Explore Manifest Schema** - [Complete reference](../api/manifest.md)
3. **Learn CLI** - [Command reference](../api/cli.md)
4. **Start Building** - Create your first custom template

## 🆘 Common Issues

### Templates Not Found
```yaml
# Wrong - missing scaffold.yaml
my-template/
├── files/
└── main.rs.hbs

# Correct
my-template/
├── scaffold.yaml    # Required
└── files/
    └── main.rs.hbs
```

### Handlebars Errors
```handlebars
<!-- Wrong - missing quotes -->
{{#if use_async}}

<!-- Correct -->
{{#if (eq use_async "true")}}
```

### Network Issues
```bash
# Template has network: deny
scaffold generate --template node-basic --apply
# Error: command blocked by network policy: npm install

# Solution: Allow network access
scaffold generate --template node-basic --apply --allow-scripts
```

## 📚 Additional Resources

- [Handlebars Documentation](https://handlebarsjs.com/guide/)
- [YAML Reference](https://yaml.org/spec/)
- [SemVer Specification](https://semver.org/)
- [CLI Reference](../api/cli.md)
- [Examples](../examples/)
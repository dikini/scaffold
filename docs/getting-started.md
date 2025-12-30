# Getting Started with Scaffold

This guide will help you get up and running with Scaffold, from installation to generating your first project.

## 📋 Prerequisites

- **Rust 1.70+** - Scaffold is written in Rust
- **Git** - For version control integration
- **Text Editor** - Any editor works, VS Code recommended
- **Terminal** - Scaffold is a command-line tool

## 🚀 Installation

### Option 1: Install from Cargo (Recommended)

```bash
cargo install scaffold
```

This installs Scaffold in your Cargo binary directory (`~/.cargo/bin/` by default).

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/dikini/scaffold
cd scaffold

# Install locally
cargo install --path .

# Or run directly
cargo run -- --help
```

### Option 3: Download Binary

1. Go to [Releases](https://github.com/dikini/scaffold/releases)
2. Download the appropriate binary for your OS
3. Extract and add to your PATH

## ⚙️ Verification

Verify your installation:

```bash
scaffold --version
scaffold --help
```

You should see version information and the help menu.

## 🎯 Quick Start

### Step 1: List Available Templates

```bash
scaffold list-templates
```

Output:
```
Available templates:

  product-planning (0.1.0)
    Product Planning with OpenCode Skills
    Complete product planning with opencode skills, beads task management, and autonomous agents
    Language: universal

  brownfield-enhancement (0.1.0)
    Brownfield Project Enhancement
    Enhances existing projects with scaffold's full product development platform, preserving legacy artifacts
    Language: universal

  rust-basic (0.1.0)
    Rust Basic Project
    A basic Rust project with CI, linting, and tests
    Language: rust

  node-basic (0.1.0)
    Node.js/TypeScript Basic Project
    A basic Node.js project with TypeScript, CI, linting, and tests
    Language: typescript

  ci-github-actions (0.1.0)
    GitHub Actions CI
    Add GitHub Actions CI workflow to an existing project
```

### Step 2: Create Variables File

Create a `vars.yaml` file to customize your project:

```yaml
# Example for Rust project
project_name: "my-awesome-app"
description: "An awesome Rust application"
author: "Your Name <you@example.com>"
license: "MIT"
use_async: false
```

### Step 3: Generate Your First Project

```bash
# Preview changes first (recommended)
scaffold generate \
  --template rust-basic \
  --out my-rust-project \
  --vars vars.yaml \
  --dry-run

# Apply changes to disk
scaffold generate \
  --template rust-basic \
  --out my-rust-project \
  --vars vars.yaml \
  --apply
```

### Step 4: Explore Generated Project

```bash
cd my-rust-project

# See what was generated
ls -la

# Try it out
cargo build
cargo test
```

## 🔧 Common Workflows

### Basic Project Generation

```bash
# Generate in current directory
scaffold generate --template rust-basic --out . --apply

# Generate with network access allowed (for npm install, etc.)
scaffold generate --template node-basic --out my-app --apply --allow-scripts

# Commit automatically after generation
scaffold generate --template rust-basic --out my-app --apply --commit
```

### Enhancing Existing Projects

```bash
# Add complete product development platform to existing project
scaffold generate --template brownfield-enhancement --out . --apply

# This preserves existing artifacts in legacy/ and adds:
# - 11 planning & task management skills
# - 13 autonomous agents
# - Beads task management system
# - Comprehensive documentation
# - CI/CD automation
```

### Adding CI to Existing Project

```bash
cd existing-project

# Add GitHub Actions
scaffold generate \
  --template ci-github-actions \
  --out . \
  --vars ci-vars.yaml \
  --apply
```

### Using JSON Output

For automation or opencode integration:

```bash
scaffold list-templates --json
scaffold generate --template rust-basic --out . --vars vars.yaml --json
```

## 📱 opencode Integration

Scaffold integrates seamlessly with opencode's command palette:

1. **Open opencode**
2. **Press** `Cmd/Ctrl + Shift + P` (or `Cmd/Ctrl + P`)
3. **Type** `scaffold init`
4. **Follow** the interactive prompts

The opencode integration automatically:
- Sets up the correct working directory
- Handles JSON output parsing
- Provides user-friendly prompts
- Shows progress and results in the UI

## 📋 Available Templates

### rust-basic
Generates a complete Rust project with:
- **Cargo.toml** with dependencies and metadata
- **src/main.rs** and **src/lib.rs** with basic structure
- **Tests** and **documentation** setup
- **GitHub Actions** CI/CD pipeline
- **Linting** configuration (clippy, rustfmt)

### node-basic
Generates a complete Node.js/TypeScript project with:
- **package.json** with modern dependencies
- **TypeScript** configuration
- **ESLint** and **Prettier** setup
- **Testing** with Jest
- **GitHub Actions** CI/CD pipeline

### ci-github-actions
Adds CI/CD to existing projects:
- **Rust**: cargo build, test, clippy, fmt
- **Node.js**: npm ci, test, lint, build
- **Python**: pip install, pytest, mypy
- **Go**: go build, test, golangci-lint
- **Generic**: Customizable workflow

## 🎛️ Configuration

### Environment Variables

```bash
# Custom templates directory
export SCAFFOLD_TEMPLATES="/path/to/my/templates"

# Default network policy
export SCAFFOLD_NETWORK_POLICY="deny"
```

### Default Network Policies

| Policy | Behavior | When to Use |
|---------|----------|--------------|
| `allow` | No restrictions | Trusted templates, quick setup |
| `deny` | Block network calls | Security-focused, offline workflows |
| `ask` | Prompt for network | Interactive environments |

## 🔍 Troubleshooting

### Template Not Found

```
Error: template not found: custom-template
```

**Solutions:**
1. Check `scaffold list-templates` for available templates
2. Use full path: `--template /path/to/template`
3. Set `SCAFFOLD_TEMPLATES` environment variable

### Permission Denied

```
Error: script execution not allowed (use --allow-scripts)
```

**Solution:**
- Add `--allow-scripts` flag if you trust the template
- Verify template scripts in `scaffold.yaml`

### Network Blocked

```
Error: command blocked by network policy: npm install
```

**Solutions:**
1. Use `--allow-scripts` with `--network-policy allow`
2. Or set `SCAFFOLD_NETWORK_POLICY=allow`
3. Verify you need network access

## 🎯 Next Steps

Now that you're comfortable with the basics:

1. **Explore Examples** - See [real-world examples](../examples/)
2. **Learn Templates** - Understand [template authoring](templates.md)
3. **Advanced Configuration** - Read [configuration guide](configuration.md)
4. **CLI Reference** - Browse [complete command docs](../api/cli.md)

## 🆘 Need Help?

- **Documentation**: [Browse all docs](README.md)
- **Issues**: [Report problems](https://github.com/dikini/scaffold/issues)
- **Discussions**: [Ask questions](https://github.com/dikini/scaffold/discussions)
- **Examples**: [See usage patterns](../examples/)
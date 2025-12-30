# Scaffold

> Language-agnostic project scaffolding tool

Scaffold is a powerful CLI tool for generating project repositories with CI, linting, tests, and release rules. It uses an agent/skill model with Handlebars templates and is designed to run from within [opencode](https://opencode.ai) (via `scaffold init` command palette entry).

## ✨ Features

- 🚀 **Language-agnostic** - Support for Rust, Node.js, Go, Python, and more
- 📦 **Template System** - Handlebars-based templates with conditions and variables
- 🔒 **Network Safety** - Configurable network policies (allow/deny/ask)
- 🛡️ **Script Guards** - Optional shell script execution with safety checks
- 🎯 **CI Integration** - Built-in GitHub Actions, GitLab CI, and more
- 📊 **JSON Output** - Native opencode integration with structured output
- ⚡ **Fast & Reliable** - Atomic file operations with preview mode

## 🚀 Quick Start

### Installation

```bash
# From source (requires Rust 1.70+)
cargo install scaffold

# Or build from this repository
git clone https://github.com/your-org/scaffold
cd scaffold
cargo install --path .
```

### Generate Your First Project

```bash
# List available templates
scaffold list-templates

# Generate a Rust project
scaffold generate --template rust-basic --out my-rust-project --apply

# Generate a Node.js project with custom variables
cat > vars.yaml << EOF
project_name: my-node-app
description: "A Node.js web application"
author: "Your Name"
license: MIT
EOF
scaffold generate --template node-basic --out my-node-app --vars vars.yaml --apply

# Generate a product planning project
cat > vars.yaml << EOF
project_name: my-product
description: "A new mobile app project"
author: "Your Name"
project_type: mobile-app
team_size: "3-5"
timeline_months: "6"
include_skills: "true"
license: MIT
EOF
scaffold generate --template product-planning --out my-product-planning --vars vars.yaml --apply
```

### Use with opencode

Scaffold integrates seamlessly with opencode's command palette:

1. Open opencode
2. Press `Cmd/Ctrl + Shift + P`
3. Type "scaffold init"
4. Follow the interactive prompts

## 📋 Available Templates

| Template | Language | Description |
|----------|----------|-------------|
| `product-planning` | Universal | Complete product planning with opencode skills, beads task management, and autonomous agents |
| `brownfield-enhancement` | Universal | Enhances existing projects with scaffold's full product development platform, preserving legacy artifacts |
| `rust-basic` | Rust | Basic Rust project with CI, linting, tests, and AGENTS.md |
| `node-basic` | TypeScript | Node.js project with TypeScript, ESLint, Jest, and AGENTS.md |
| `ci-github-actions` | Any | Add GitHub Actions CI to existing project |

## 🛠️ Commands

### `scaffold list-templates`
List all available templates.

```bash
scaffold list-templates [--json]
```

### `scaffold validate`
Validate a template manifest.

```bash
scaffold validate --template <id|path> [--vars vars.yaml] [--json]
```

### `scaffold generate`
Generate a project from a template.

```bash
scaffold generate \
  --template <id|path> \
  --out <directory> \
  [--vars vars.yaml] \
  [--dry-run] \
  [--apply] \
  [--allow-scripts] \
  [--commit] \
  [--json]
```

**Key Options:**
- `--dry-run`: Preview changes without applying
- `--apply`: Write changes to disk
- `--allow-scripts`: Enable shell script execution
- `--commit`: Git commit after generation
- `--json`: Output structured JSON for opencode

### `scaffold init`
Initialize a new project interactively.

```bash
scaffold init [--template <id>] [--interactive] [--out .] [--json]
```

### `scaffold test`
Run tests for a generated project.

```bash
scaffold test --target <directory> [--json]
```

## 📝 Template Variables

Create a `vars.yaml` file to customize template generation:

```yaml
# Rust project example
project_name: "my-awesome-app"
description: "An awesome Rust application"
author: "Your Name <you@example.com>"
license: "MIT"
use_async: true

# Node.js project example
project_name: "my-node-app"
description: "A Node.js web application"
author: "Your Name"
license: "MIT"
use_eslint: true
use_prettier: true
```

## 🔧 Configuration

Scaffold can be configured via environment variables:

```bash
# Custom templates directory
export SCAFFOLD_TEMPLATES="/path/to/my/templates"

# Default network policy
export SCAFFOLD_NETWORK_POLICY="deny"
```

## 📚 Documentation

- [Getting Started Guide](docs/getting-started.md) - Detailed setup and usage
- [Template Authoring](docs/templates.md) - Create your own templates
- [CLI Reference](docs/api/cli.md) - Complete command reference
- [Manifest Schema](docs/api/manifest.md) - Template manifest specification
- [Examples](docs/examples/) - Real-world usage examples

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/your-org/scaffold
cd scaffold

# Install dependencies
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Adding Templates

1. Create a new directory in `templates/`
2. Add a `scaffold.yaml` manifest
3. Add template files in `files/`
4. Test with `scaffold validate --template your-template`
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [Handlebars](https://handlebarsjs.com/) for templating
- Inspired by various project generators in the ecosystem

## 🔗 Links

- [Repository](https://github.com/your-org/scaffold)
- [Documentation](https://github.com/your-org/scaffold/docs)
- [Issues](https://github.com/your-org/scaffold/issues)
- [Discussions](https://github.com/your-org/scaffold/discussions)
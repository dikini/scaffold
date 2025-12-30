# Configuration

Configure Scaffold behavior through environment variables and configuration files.

## 🔧 Environment Variables

### SCAFFOLD_TEMPLATES
**Type**: `path` | Default: `templates/` (relative to binary)

Custom directory containing template collections.

```bash
# Use custom templates directory
export SCAFFOLD_TEMPLATES="/home/user/.scaffold/templates"

# Use project-relative templates
export SCAFFOLD_TEMPLATES="./templates"

# Use multiple directories (colon-separated)
export SCAFFOLD_TEMPLATES="/path/to/templates:/path/to/more-templates"
```

**Directory Structure**:
```
my-templates/
├── rust-basic/
│   ├── scaffold.yaml
│   └── files/
├── node-web/
│   ├── scaffold.yaml
│   └── files/
└── shared/
    ├── common-files/
    └── snippets/
```

**Discovery Order**:
1. Directories in `SCAFFOLD_TEMPLATES`
2. Built-in templates (if any)
3. Current directory `templates/`

### SCAFFOLD_NETWORK_POLICY
**Type**: `enum` | Default: `allow`

Default network policy when not specified in template.

```bash
# Deny network by default (secure)
export SCAFFOLD_NETWORK_POLICY="deny"

# Allow network (default)
export SCAFFOLD_NETWORK_POLICY="allow"

# Ask for network operations
export SCAFFOLD_NETWORK_POLICY="ask"
```

**Values**:
- `allow`: No network restrictions
- `deny`: Block all network commands
- `ask`: Prompt before allowing network operations

### NO_COLOR
**Type**: `bool` | Default: undefined

Disable colored output.

```bash
export NO_COLOR=1
scaffold generate --template rust-basic --out my-project --apply
```

### RUST_LOG
**Type**: `string` | Default: `info`

Control Rust logging level for debugging.

```bash
# Debug logging
export RUST_LOG=debug
scaffold generate --template rust-basic --out my-project --apply

# Error logging only
export RUST_LOG=error
scaffold list-templates
```

**Levels**: `error`, `warn`, `info`, `debug`, `trace`

### RUST_BACKTRACE
**Type**: `enum` | Default: `0`

Enable Rust backtraces for debugging crashes.

```bash
# Full backtraces
export RUST_BACKTRACE=1
scaffold generate --template rust-basic --out my-project --apply

# Short backtraces
export RUST_BACKTRACE=short
scaffold generate --template rust-basic --out my-project --apply
```

## 📁 Configuration Files

### Configuration File Location
Scaffold looks for configuration in this order:
1. `./.scaffold.yaml` (project-specific)
2. `~/.scaffold/config.yaml` (user-specific)
3. `/etc/scaffold/config.yaml` (system-wide)

### Configuration Schema
```yaml
# .scaffold/config.yaml
# User-specific Scaffold configuration

# Template directories
templates:
  - "/home/user/.scaffold/templates"
  - "./company-templates"
  - "/opt/scaffold/templates"

# Default behaviors
defaults:
  network_policy: "deny"        # allow, deny, ask
  auto_commit: false            # Auto-commit after generation
  allow_scripts: false          # Default script permission
  
# Output preferences
output:
  json: false                  # Default to JSON output
  preview: true                # Always show preview before apply
  
# Editor integration
editor:
  name: "vscode"             # vscode, vim, emacs
  command: "code"             # Editor command
  args: ["--new-window"]      # Editor arguments
```

### Template Configuration
Configure specific template behaviors:

```yaml
# Template-specific defaults
template_defaults:
  rust-basic:
    network_policy: "deny"
    license: "MIT"
  
  node-basic:
    network_policy: "allow"
    author: "${USER}"
    license: "MIT"
```

## 🎯 Common Configurations

### Security-Focused Setup
```bash
# ~/.scaffold/config.yaml
defaults:
  network_policy: "deny"
  allow_scripts: false
  auto_commit: false

templates:
  - "/secure/templates/only"
```

### Development-Focused Setup
```bash
# ~/.scaffold/config.yaml  
defaults:
  network_policy: "allow"
  allow_scripts: true
  auto_commit: true

templates:
  - "./dev-templates"
  - "../shared-templates"
```

### Team Environment Setup
```bash
# ~/.scaffold/config.yaml
templates:
  - "/company/scaffold-templates"
  - "${HOME}/.scaffold/personal-templates"

template_defaults:
  "*":
    author: "${USER}"
    company: "ACME Corp"
    license: "company-proprietary"
```

### CI/CD Integration
```bash
# ~/.scaffold/config.yaml
output:
  json: true                  # Always JSON output for automation
  
editor:
  name: "vscode"
  command: "code"
  args: ["--wait"]            # Wait for file to close
```

## 🔧 Command Line Override

Environment variables and config files can be overridden by command-line options:

### Override Priority (highest to lowest)
1. **Command-line flags** (`--apply`, `--allow-scripts`)
2. **Environment variables** (`SCAFFOLD_NETWORK_POLICY`)
3. **Configuration files** (`~/.scaffold/config.yaml`)
4. **Template defaults** (`scaffold.yaml`)
5. **Scaffold defaults** (built-in)

### Example Override Chain
```bash
# Config file sets network_policy: deny
# Environment variable sets network_policy: allow  
# Command line sets --allow-scripts (overrides script restrictions)

scaffold generate \
  --template node-basic \
  --out my-app \
  --apply \
  --allow-scripts    # This overrides everything
```

## 📱 Shell Integration

### Bash Completions
Enable bash tab completion:

```bash
# Add to ~/.bashrc or ~/.bash_profile
eval "$(scaffold --completion=bash)"

# Or source directly
source <(scaffold --completion=bash)
```

### Zsh Completions
Enable zsh tab completion:

```bash
# Add to ~/.zshrc
eval "$(scaffold --completion=zsh)"

# Or source directly
source <(scaffold --completion=zsh)
```

### Fish Completions
Enable fish tab completion:

```bash
# Add to ~/.config/fish/config.fish
scaffold --completion=fish | source

# Or save and source
scaffold --completion=fish > ~/.config/fish/scaffold_completions.fish
```

## 🔍 Debugging Configuration

### Check Current Configuration
```bash
# Show effective configuration
scaffold config show

# Show where config files were loaded
scaffold config show --sources

# Validate configuration
scaffold config validate
```

### Debug Mode
Enable comprehensive debugging:

```bash
# Enable debug logging and backtraces
export RUST_LOG=debug
export RUST_BACKTRACE=1
export RUST_LOG=debug

# Show configuration loading
scaffold list-templates
```

### Common Issues

#### Template Not Found
```bash
# Check template paths
scaffold config show
# Look for 'templates' section

# Verify directory exists
ls -la "$SCAFFOLD_TEMPLATES"
```

#### Permission Denied
```bash
# Check effective policy
scaffold config show | grep network_policy

# Verify script permissions
scaffold config show | grep allow_scripts
```

#### Configuration Not Loading
```bash
# Check config file syntax
scaffold config validate

# Show config sources
scaffold config show --sources
```

## 🎯 Best Practices

### Security
1. **Use `network_policy: deny`** for security environments
2. **Set `allow_scripts: false`** by default
3. **Restrict template sources** to trusted directories
4. **Audit templates** before using in production

### Team Workflows
1. **Share config via version control** (with secrets excluded)
2. **Use environment variables** for user-specific settings
3. **Standardize template locations** across team
4. **Document custom templates** with README files

### Development
1. **Use separate config** for development vs production
2. **Enable debug logging** when troubleshooting
3. **Test configuration changes** in isolated environment
4. **Version control config** changes

## 🔗 See Also

- [Getting Started Guide](../getting-started.md) - Basic usage
- [Template Authoring](../templates.md) - Template creation
- [CLI Reference](api/cli.md) - Command options
- [Examples](../examples/) - Real-world configurations
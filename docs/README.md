# Scaffold Documentation

Welcome to the Scaffold documentation! This guide covers everything from basic usage to advanced template authoring.

## 📚 Table of Contents

### Getting Started
- [Getting Started Guide](getting-started.md) - New to Scaffold? Start here
- [Installation](getting-started.md#installation) - How to install Scaffold
- [Quick Start](getting-started.md#quick-start) - Generate your first project

### Core Concepts
- [Templates](templates.md) - Understanding and using templates
- [Template Authoring](templates.md#authoring) - Create your own templates
- [Manifest Schema](api/manifest.md) - Template manifest specification

### Reference
- [CLI Reference](api/cli.md) - Complete command documentation
- [Configuration](configuration.md) - Environment variables and config
- [Examples](examples/) - Real-world usage examples

### Advanced
- [Network Policies](templates.md#network-policies) - Control network access
- [Script Execution](templates.md#script-execution) - Safe script running
- [opencode Integration](getting-started.md#opencode-integration) - IDE integration

## 🚀 Quick Links

### New Users
1. [Installation](getting-started.md#installation)
2. [Generate Your First Project](getting-started.md#generate-your-first-project)
3. [Available Templates](getting-started.md#available-templates)

### Template Authors
1. [Template Structure](templates.md#template-structure)
2. [Manifest Reference](api/manifest.md)
3. [Handlebars Guide](templates.md#handlebars-usage)
4. [Best Practices](templates.md#best-practices)

### Advanced Users
1. [Configuration Options](configuration.md)
2. [Network Policies](templates.md#network-policies)
3. [Script Execution](templates.md#script-execution)
4. [JSON Output Mode](getting-started.md#json-output-mode)

## 🛠️ Need Help?

- **GitHub Issues**: [Report bugs or request features](https://github.com/dikini/scaffold/issues)
- **GitHub Discussions**: [Ask questions and share ideas](https://github.com/dikini/scaffold/discussions)
- **Examples**: Browse [real-world examples](examples/)

## 📖 Documentation Structure

```
docs/
├── README.md              # This file - documentation index
├── getting-started.md     # Detailed getting started guide
├── templates.md           # Template usage and authoring
├── configuration.md       # Configuration options
├── examples/              # Usage examples
│   ├── rust-project.md
│   ├── node-project.md
│   └── custom-template.md
└── api/                   # API documentation
    ├── cli.md             # CLI reference
    └── manifest.md        # Manifest schema reference
```

## 🔄 Keeping Updated

This documentation is versioned with the Scaffold codebase. For the latest version, check the [GitHub repository](https://github.com/dikini/scaffold).

## 🤝 Contributing to Documentation

Found an error or want to improve the docs? We welcome contributions!

1. Fork the repository
2. Make your changes
3. Test the documentation locally
4. Submit a pull request

See our [Contributing Guide](../CONTRIBUTING.md) for more details.
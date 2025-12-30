# Contributing to Scaffold

We welcome contributions to Scaffold! This guide covers everything you need to know to contribute effectively.

## 🎯 Types of Contributions

We accept these types of contributions:

### 🐛 Bug Reports
Report bugs using [GitHub Issues](https://github.com/your-org/scaffold/issues).

### ✨ Feature Requests
Request new features using [GitHub Issues](https://github.com/your-org/scaffold/issues).

### 📝 Documentation
Improve documentation through pull requests.

### 🔧 Code Contributions
Add features, fix bugs, or improve performance.

### 🎨 Templates
Create new templates or improve existing ones.

## 🚀 Getting Started

### Development Setup

```bash
# Fork and clone
git clone https://github.com/yourusername/scaffold
cd scaffold

# Install dependencies
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt
```

### Development Workflow

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Write code
   - Add tests
   - Update documentation
   - Run `cargo fmt` and `cargo clippy`

3. **Test Changes**
   ```bash
   cargo test
   cargo test --package scaffold --test integration_test
   ```

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add your feature"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   # Create pull request on GitHub
   ```

## 📋 Code Standards

### Formatting
We use standard Rust formatting:

```bash
cargo fmt
```

All code must be formatted before committing.

### Linting
We enforce high code quality with Clippy:

```bash
cargo clippy -- -D warnings
```

No warnings are allowed in pull requests.

### Testing
Write comprehensive tests:

- **Unit tests** for individual functions
- **Integration tests** for CLI commands
- **Property-based tests** where applicable

Test coverage should be maintained above 80%.

### Documentation
- **Code comments** for complex logic
- **Doc comments** for public APIs
- **README updates** for user-facing changes

## 🏗️ Architecture Overview

Scaffold follows this package structure:

```
scaffold/
├── cmd/scaffold/           # CLI binary
├── pkg/                   # Library crates
│   ├── manifest/           # Template parsing and validation
│   ├── renderer/           # Handlebars rendering
│   ├── fsutil/             # File system operations
│   ├── executor/           # Script execution
│   └── opencode/           # JSON output adapter
├── templates/             # Template collection
├── tests/                # Integration tests
└── docs/                 # Documentation
```

### Core Libraries

#### `manifest`
Handles template manifests:
- YAML parsing and validation
- Input type definitions
- Condition evaluation
- Schema compliance

#### `renderer`
Template rendering engine:
- Handlebars integration
- File processing
- Diff generation
- Preview mode

#### `fsutil`
File system utilities:
- Atomic operations
- Temp directory management
- File copying and creation
- Path manipulation

#### `executor`
Script execution with safety:
- Network policy enforcement
- Command filtering
- Error handling
- Security controls

#### `opencode`
Integration adapter:
- JSON output formatting
- Status reporting
- Error serialization

## 📝 Template Contributions

### Template Guidelines

Create templates following these standards:

#### Structure
```
template-name/
├── scaffold.yaml          # Manifest
└── files/                # Template files
    ├── src/
    ├── tests/
    └── docs/
```

#### Quality Standards
- **Security**: Default to `network: deny`
- **Usability**: Provide sensible defaults
- **Maintainability**: Clear, well-documented code
- **Compatibility**: Work across common environments

#### Testing
Test your template thoroughly:

```bash
# Validate template
scaffold validate --template your-template

# Test generation
scaffold generate \
  --template your-template \
  --out test-project \
  --vars test-vars.yaml \
  --dry-run

# Test generated project
scaffold generate \
  --template your-template \
  --out test-project \
  --vars test-vars.yaml \
  --apply

# Run tests on generated project
scaffold test --target test-project
```

### Template Submission

1. **Fork Repository**
2. **Create Template Directory**: `templates/your-template/`
3. **Add Template Files** following structure
4. **Add Documentation**: `templates/your-template/README.md`
5. **Test Thoroughly**: Manual and automated testing
6. **Submit Pull Request** with:
   - Template description
   - Usage examples
   - Test results
   - Screenshots (if applicable)

## 🔧 Code Contribution Process

### Bug Fixes

1. **Create Issue**: Describe bug with reproduction steps
2. **Create Branch**: `git checkout -b fix/issue-number-description`
3. **Implement Fix**: Minimum code to fix the issue
4. **Add Tests**: Ensure fix works and prevent regression
5. **Update Docs**: If behavior changes
6. **Submit PR**: Reference issue in description

### Feature Development

1. **Create Issue**: Discuss proposal before implementation
2. **Design Phase**: Consider API, backward compatibility
3. **Create Branch**: `git checkout -b feature/feature-name`
4. **Implement**: Follow existing patterns and conventions
5. **Test**: Unit tests, integration tests, manual testing
6. **Document**: Update relevant documentation
7. **Submit PR**: Include examples and test results

### Refactoring

1. **Identify Area**: Code quality, performance, maintainability
2. **Plan Changes**: Ensure no behavior changes
3. **Create Branch**: `git checkout -b refactor/area-name`
4. **Implement**: Improve internal structure
5. **Test**: Cover refactored code
6. **Update Docs**: If public APIs change
7. **Submit PR**: Explain refactoring rationale

## 📚 Documentation Contributions

### Types of Documentation

#### User Documentation
- **Getting Started**: First-time user experience
- **Tutorials**: Step-by-step guides
- **Examples**: Real-world usage patterns
- **FAQ**: Common questions and issues

#### Developer Documentation
- **API Reference**: Complete command and library docs
- **Template Authoring**: Template creation guide
- **Architecture**: System design and patterns
- **Contributing**: This file

### Documentation Process

1. **Preview Changes**: Use `mkdocs serve` locally
2. **Check Links**: Ensure all links work
3. **Review Examples**: Test code examples
4. **Update TOC**: Add new sections to table of contents
5. **Submit PR**: Include screenshots of changes

## 🧪 Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    
    #[test]
    fn test_cli_command() {
        let temp = assert_fs::TempDir::new().unwrap();
        let output = Command::new("scaffold")
            .arg("generate")
            .arg("--template")
            .arg("test-template")
            .arg("--out")
            .arg(temp.path())
            .output()
            .expect("Failed to execute scaffold");
        
        assert!(output.status.success());
        assert!(temp.path().join("generated-file").exists());
    }
}
```

### Test Organization

```
tests/
├── unit/                    # Unit tests for each crate
│   ├── manifest_tests.rs
│   ├── renderer_tests.rs
│   └── ...
└── integration/               # End-to-end tests
    ├── cli_tests.rs
    └── template_tests.rs
```

## 🔍 Code Review Process

### Review Checklist

Reviewers check for:

#### Code Quality
- [ ] Code follows Rust conventions
- [ ] No clippy warnings
- [ ] Proper error handling
- [ ] No unnecessary complexity

#### Testing
- [ ] Tests cover new functionality
- [ ] Test cases cover edge cases
- [ ] Integration tests verify workflow
- [ ] Tests pass locally

#### Documentation
- [ ] Code comments for complex logic
- [ ] Public API documented
- [ ] User documentation updated
- [ ] Examples provided

#### Security
- [ ] No hardcoded secrets
- [ ] Input validation
- [ ] Safe defaults (network: deny)
- [ ] Proper error messages

### Review Guidelines

1. **Be Constructive**: Focus on improvement, not criticism
2. **Be Specific**: Provide clear, actionable feedback
3. **Be Thorough**: Check all aspects (code, tests, docs)
4. **Be Responsive**: Address reviews promptly
5. **Be Helpful**: Suggest improvements and alternatives

## 📦 Release Process

### Version Management

We use [Semantic Versioning](https://semver.org/):

- **Major** (X.0.0): Breaking changes
- **Minor** (X.Y.0): New features, backward compatible
- **Patch** (X.Y.Z): Bug fixes, backward compatible

### Release Checklist

Before releasing:

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version number updated
- [ ] Security review completed
- [ ] Performance tests pass
- [ ] Release notes prepared

### Release Process

```bash
# Update version
cargo version patch  # or minor/major

# Update CHANGELOG
echo "## [$(cargo version | cut -d' ' -f2)]" >> CHANGELOG.md

# Create release tag
git tag -a v$(cargo version) -m "Release $(cargo version)"

# Push to main
git push origin main
git push origin v$(cargo version)

# Publish to crates.io
cargo publish
```

## 🔗 Resources

### Development Resources
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/) - Code quality
- [Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html) - Testing patterns
- [API Guidelines](https://rust-lang.github.io/api-guidelines/) - API design
- [Conventional Commits](https://www.conventionalcommits.org/) - Commit messages

### Tools and Libraries
- [Handlebars](https://handlebarsjs.com/) - Template engine
- [Serde](https://serde.rs/) - Serialization
- [Clap](https://clap.rs/) - CLI parsing
- [Tokio](https://tokio.rs/) - Async runtime
- [Similar](https://github.com/mitsuhiko/similar-rs) - Diff generation

### Project Resources
- [Repository](https://github.com/your-org/scaffold) - Source code
- [Issues](https://github.com/your-org/scaffold/issues) - Report bugs
- [Discussions](https://github.com/your-org/scaffold/discussions) - Ask questions
- [Documentation](https://github.com/your-org/scaffold/docs) - Full docs

## 🙏 Getting Help

### For Contributors
- **GitHub Discussions**: [Ask questions](https://github.com/your-org/scaffold/discussions)
- **Issues**: [Report problems](https://github.com/your-org/scaffold/issues)
- **Email**: maintainer@example.com (for sensitive issues)

### For Users
- **Documentation**: [Browse all docs](https://github.com/your-org/scaffold/docs)
- **Examples**: [See usage patterns](https://github.com/your-org/scaffold/tree/main/docs/examples)
- **Issues**: [Report bugs](https://github.com/your-org/scaffold/issues)
- **Discussions**: [Ask questions](https://github.com/your-org/scaffold/discussions)

## 📄 Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please read our full [Code of Conduct](CODE_OF_CONDUCT.md).

---

Thank you for contributing to Scaffold! Your contributions help make this project better for everyone. 🚀
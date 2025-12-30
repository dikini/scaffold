# Custom Template Example

Learn to create your own Scaffold template from scratch.

## 🎯 Goal

Create a "Python CLI Tool" template that generates:
- CLI application with `clap`
- Proper project structure
- Testing with `pytest`
- CI/CD with GitHub Actions
- Documentation with `mkdocs`

## 📁 Template Creation Process

### Step 1: Create Template Directory

```bash
# Create template structure
mkdir -p templates/python-cli/files/src
cd templates/python-cli
```

### Step 2: Write Template Manifest

```yaml
# templates/python-cli/scaffold.yaml
id: python-cli
name: Python CLI Tool
description: A Python CLI application with argparse, tests, and documentation
version: "1.0.0"
language: python
ci_provider: github_actions
network: deny

inputs:
  - name: project_name
    type: string
    prompt: "Project name?"
    required: true
    default: "my-cli-tool"
  
  - name: description
    type: string
    prompt: "CLI description?"
    default: "A Python CLI tool"
  
  - name: author
    type: string
    prompt: "Author name?"
    default: "Your Name <you@example.com>"
  
  - name: license
    type: enum
    enum_values:
      - MIT
      - Apache-2.0
      - GPL-3.0
      - none
    default: MIT
  
  - name: python_version
    type: enum
    enum_values:
      - "3.10"
      - "3.11"
      - "3.12"
    default: "3.11"
  
  - name: use_poetry
    type: bool
    prompt: "Use Poetry for dependency management?"
    default: "true"
  
  - name: include_docs
    type: bool
    prompt: "Include documentation setup?"
    default: "true"

steps:
  - type: render_template
    src: pyproject.toml.hbs
    dest: pyproject.toml
  
  - type: render_template
    src: src/cli.py.hbs
    dest: src/{{project_name}}/__main__.py
  
  - type: render_template
    src: src/__init__.py.hbs
    dest: src/{{project_name}}/__init__.py
  
  - type: render_template
    src: tests/test_cli.py.hbs
    dest: tests/test_cli.py
  
  - type: create_file
    dest: README.md
    content: |
      # {{project_name}}
      
      {{description}}
      
      ## Installation
      
      {{#if use_poetry}}
      ```bash
      poetry install
      poetry run {{project_name}}
      ```
      {{else}}
      ```bash
      pip install -e .
      python -m {{project_name}}
      ```
      {{/if}}
      
      ## Usage
      
      ```bash
      {{project_name}} --help
      ```
  
  - type: create_file
    dest: .gitignore
    content: |
      # Byte-compiled / optimized / DLL files
      __pycache__/
      *.py[cod]
      *$py.class
      
      # C extensions
      *.so
      
      # Distribution / packaging
      .Python
      build/
      develop-eggs/
      dist/
      downloads/
      eggs/
      .eggs/
      lib/
      lib64/
      parts/
      sdist/
      var/
      wheels/
      *.egg-info/
      *.egg
      MANIFEST
      
      # PyInstaller
      *.manifest
      *.spec
      
      # Unit test / coverage reports
      htmlcov/
      .tox/
      .coverage
      .coverage.*
      .cache
      nosetests.xml
      coverage.xml
      *.cover
      *.py,cover
      .hypothesis/
      .pytest_cache/
      
      # Environments
      .env
      .venv
      env/
      venv/
      ENV/
      env.bak/
      venv.bak/
  
  {{#if include_docs}}
  - type: render_template
    src: mkdocs.yml.hbs
    dest: mkdocs.yml
  
  - type: create_file
    dest: docs/index.md
    content: |
      # {{project_name}} Documentation
      
      Welcome to the documentation for {{project_name}}.
      
      ## Installation
      
      {{#if use_poetry}}
      ```bash
      poetry install
      poetry install
      ```
      {{else}}
      ```bash
      pip install {{project_name}}
      ```
      {{/if}}
      
      ## Quick Start
      
      ```bash
      {{project_name}} --help
      ```
  {{/if}}
  
  - type: copy
    src: .github
    dest: .github

tests:
  {{#if use_poetry}}
  - "poetry install"
  - "poetry run pytest"
  {{else}}
  - "pip install -e ."
  - "python -m pytest"
  {{/if}}
  - "python -m mypy . --ignore-missing-imports"
  - "python -m ruff check ."
  - "python -m ruff format . --check"
```

### Step 3: Create Template Files

```toml
<!-- templates/python-cli/files/pyproject.toml.hbs -->
[build-system]
requires = ["setuptools>=61.0"]
build-backend = "setuptools.build_meta"

[project]
name = "{{project_name}}"
version = "0.1.0"
description = "{{description}}"
authors = [
    {name = "{{author}}", email = "you@example.com"},
]
readme = "README.md"
license = {file = "LICENSE"}
requires-python = ">{{python_version}}"
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: {{license}} License",
    "Programming Language :: Python :: {{python_version}}",
    "Programming Language :: Python :: 3",
]

{{#if use_poetry}}
[tool.poetry]
name = "{{project_name}}"
version = "0.1.0"
description = "{{description}}"
authors = ["{{author}}"]
license = "{{license}}"
[tool.poetry.dependencies]
click = "^8.0"
[tool.poetry.group.dev.dependencies]
pytest = "^7.0"
mypy = "^1.0"
ruff = "^0.1"
{{else}}
dependencies = [
    "click>=8.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "mypy>=1.0",
    "ruff>=0.1",
]
{{/if}}

[project.scripts]
{{project_name}} = "{{project_name}}.cli:main"

[tool.setuptools.packages.find]
where = ["src"]

[tool.pytest.ini_options]
testpaths = ["tests"]
python_files = ["test_*.py"]
python_classes = ["Test*"]
python_functions = ["test_*"]
```

```python
<!-- templates/python-cli/files/src/cli.py.hbs -->
#!/usr/bin/env python3
"""{{description}}"""

import argparse
import sys
from pathlib import Path

try:
    from .{{project_name}} import main as cli_main
except ImportError:
    sys.path.insert(0, str(Path(__file__).parent.parent))
    from {{project_name}} import main as cli_main


def create_parser() -> argparse.ArgumentParser:
    """Create the argument parser."""
    parser = argparse.ArgumentParser(
        description="{{description}}",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    
    parser.add_argument(
        "--version",
        action="version",
        version="%(prog)s 0.1.0"
    )
    
    parser.add_argument(
        "command",
        nargs="?",
        choices=["hello", "version"],
        help="Command to execute",
        default="hello"
    )
    
    parser.add_argument(
        "--name",
        default="World",
        help="Name to greet"
    )
    
    return parser


def main() -> int:
    """Main entry point."""
    parser = create_parser()
    args = parser.parse_args()
    
    if args.command == "version":
        print("{{project_name}} 0.1.0")
        return 0
    
    return cli_main(args)


if __name__ == "__main__":
    sys.exit(main())
```

```python
<!-- templates/python-cli/files/src/__main__.py.hbs -->
"""Main CLI functionality."""

import sys
from pathlib import Path


def hello_command(args) -> int:
    """Handle hello command."""
    print(f"Hello, {args.name}!")
    return 0


def version_command(args) -> int:
    """Handle version command."""
    print("{{project_name}} 0.1.0")
    return 0


def main(args) -> int:
    """Main CLI functionality."""
    if args.command == "hello":
        return hello_command(args)
    elif args.command == "version":
        return version_command(args)
    else:
        print(f"Unknown command: {args.command}")
        return 1


# For when module is run directly
if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", default="World")
    parser.add_argument("command", choices=["hello", "version"])
    
    # Create a minimal args object for testing
    class Args:
        def __init__(self):
            self.name = "World"
            self.command = "hello"
    
    main(Args())
```

```python
<!-- templates/python-cli/files/src/__init__.py.hbs -->
"""{{project_name}} package."""

__version__ = "0.1.0"
__author__ = "{{author}}"

try:
    from .cli import main
except ImportError:
    # Fallback for development
    import sys
    from pathlib import Path
    
    sys.path.insert(0, str(Path(__file__).parent))
    from cli import main

__all__ = ["main", "__version__"]
```

```python
<!-- templates/python-cli/files/tests/test_cli.py.hbs -->
"""Tests for {{project_name}} CLI."""

import pytest
import sys
from pathlib import Path

# Add src to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent))

try:
    from {{project_name}} import main
except ImportError:
    pytest.skip("Cannot import main module")


class TestCLI:
    """Test CLI functionality."""
    
    def test_hello_command(self, monkeypatch):
        """Test hello command."""
        # Mock command line arguments
        monkeypatch.setattr(sys, "argv", ["test", "hello", "--name", "Test"])
        
        class Args:
            name = "Test"
            command = "hello"
        
        result = main.hello_command(Args())
        assert result == 0
    
    def test_version_command(self):
        """Test version command."""
        class Args:
            command = "version"
        
        result = main.version_command(Args())
        assert result == 0
    
    def test_main_unknown_command(self, monkeypatch):
        """Test main with unknown command."""
        monkeypatch.setattr(sys, "argv", ["test", "unknown"])
        
        class Args:
            name = "World"
            command = "unknown"
        
        result = main.main(Args())
        assert result == 1


class TestModule:
    """Test module metadata."""
    
    def test_version_exists(self):
        """Test that version is defined."""
        from {{project_name}} import __version__
        assert __version__ == "0.1.0"
    
    def test_author_exists(self):
        """Test that author is defined."""
        from {{project_name}} import __author__
        assert "{{author}}" in __author__


if __name__ == "__main__":
    pytest.main([__file__])
```

```yaml
<!-- templates/python-cli/files/mkdocs.yml.hbs -->
site_name: {{project_name}}
site_description: {{description}}
repo_url: https://github.com/username/{{project_name}}

nav:
  - Home: index.md
  - Installation: installation.md
  - Usage: usage.md
  - API: api.md

theme: readthedocs

plugins:
  - search
  - mkdocstrings

markdown_extensions:
  - codehilite
  - admonition
```

### Step 4: Add GitHub Actions Workflow

```yaml
<!-- templates/python-cli/files/.github/workflows/ci.yml.hbs -->
name: CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

env:
  PYTHON_VERSION: {{python_version}}

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: [{{python_version}}]

    steps:
      - uses: actions/checkout@v4

      - name: Set up Python ${{ matrix.python-version }}
        uses: actions/setup-python@v4
        with:
          python-version: ${{ matrix.python-version }}

      - name: Cache pip dependencies
        uses: actions/cache@v4
        with:
          path: ~/.cache/pip
          key: ${{ runner.os }}-pip-${{ hashFiles('**/requirements*.txt') }}
          restore-keys: |
            ${{ runner.os }}-pip-

      {{#if use_poetry}}
      - name: Install Poetry
        run: |
          curl -sSL https://install.python-poetry.org | python3 -
      
      - name: Cache Poetry dependencies
        uses: actions/cache@v4
        with:
          path: ~/.cache/poetry
          key: ${{ runner.os }}-poetry-${{ hashFiles('**/poetry.lock') }}
          restore-keys: |
            ${{ runner.os }}-poetry-

      - name: Install dependencies
        run: poetry install

      - name: Run tests
        run: |
          poetry run pytest
          poetry run mypy .
          poetry run ruff check .
          poetry run ruff format . --check
      {{else}}
      - name: Install dependencies
        run: |
          python -m pip install --upgrade pip
          python -m pip install -e .

      - name: Run tests
        run: |
          python -m pytest
          python -m mypy . --ignore-missing-imports
          python -m ruff check .
          python -m ruff format . --check
      {{/if}}
```

### Step 5: Test the Template

```bash
# Validate template
scaffold validate --template ./python-cli

# Create test variables
cat > test-vars.yaml << 'EOF'
project_name: "my-python-cli"
description: "A Python CLI tool for testing"
author: "Test Author <test@example.com>"
license: "MIT"
python_version: "3.11"
use_poetry: true
include_docs: true
EOF

# Generate test project
scaffold generate \
  --template ./python-cli \
  --out test-cli \
  --vars test-vars.yaml \
  --dry-run

# Review output, then apply
scaffold generate \
  --template ./python-cli \
  --out test-cli \
  --vars test-vars.yaml \
  --apply

# Test generated project
cd test-cli
{{#if use_poetry}}
poetry install
poetry run pytest
{{else}}
pip install -e .
python -m pytest
{{/if}}
```

## 🧪 Template Testing

### Unit Testing Template

```bash
# Test with different inputs
scaffold validate --template ./python-cli --vars inputs/all-vars.yaml

# Test edge cases
scaffold validate --template ./python-cli --vars inputs/edge-cases.yaml

# Generate and verify structure
scaffold generate --template ./python-cli --out edge-test --apply
find edge-test -type f
```

### Integration Testing Template

```bash
# Generate with CI
scaffold generate \
  --template ./python-cli \
  --out integration-test \
  --vars ci-vars.yaml \
  --apply

# Run the actual tests
cd integration-test
{{#if use_poetry}}
poetry run pytest
{{else}}
python -m pytest
{{/if}}

# Test CI locally
act -j ubuntu-latest
```

## 🚀 Publishing Your Template

### Prepare for Distribution

```bash
# Create template repository
mkdir -p scaffold-templates
cp -r python-cli scaffold-templates/

cd scaffold-templates

# Add documentation
echo "# Python CLI Template" > python-cli/README.md
echo "Generate Python CLI applications with best practices." >> python-cli/README.md

# Create usage example
cat > python-cli/example.yaml << 'EOF'
project_name: "my-tool"
description: "My custom Python tool"
author: "Developer Name"
license: "MIT"
use_poetry: true
EOF
```

### Share Template

#### Option 1: Local Templates
```bash
# Set custom template directory
export SCAFFOLD_TEMPLATES="~/scaffold-templates"

# Use your template
scaffold list-templates
scaffold generate --template python-cli --out my-tool --apply
```

#### Option 2: GitHub Repository
```bash
# Create GitHub repository
gh repo create scaffold-templates --public --clone=false

# Push template
cd scaffold-templates
git init
git add .
git commit -m "Add Python CLI template"
git remote add origin https://github.com/yourname/scaffold-templates.git
git push -u origin main
```

#### Option 3: Template Registry
```yaml
# template-metadata.yaml (for registry submission)
name: "python-cli"
version: "1.0.0"
description: "Python CLI application template"
author: "Your Name <you@example.com>"
license: "MIT"
repository: "https://github.com/yourname/scaffold-templates"
homepage: "https://github.com/yourname/scaffold-templates/tree/main/python-cli"
tags: ["python", "cli", "click", "poetry"]
```

## 🎯 Advanced Template Features

### Conditional Steps

```yaml
steps:
  - type: render_template
    src: pyproject.toml.poetry.hbs
    dest: pyproject.toml
    when: "use_poetry == 'true'"
  
  - type: render_template
    src: pyproject.toml.setuptools.hbs
    dest: pyproject.toml
    when: "use_poetry != 'true'"
```

### Complex Variable Logic

```handlebars
<!-- templates/python-cli/files/pyproject.toml.hbs -->
[project]
name = "{{project_name}}"
{{#if use_poetry}}
[tool.poetry.dependencies]
click = "^8.0"
[tool.poetry.group.dev.dependencies]
pytest = "^7.0"
mypy = "^1.0"
ruff = "^0.1"
{{else}}
dependencies = [
    "click>=8.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "mypy>=1.0",
    "ruff>=0.1",
]
{{/if}}
```

### Post-apply Customization

```yaml
post_apply:
  - type: create_file
    dest: .env.example
    content: |
      # Environment variables for {{project_name}}
      DEBUG=false
      LOG_LEVEL=info
  
  - type: run_shell
    cmd: "git init && git add . && git commit -m 'Initial commit: {{project_name}} CLI'"
    when: "initialize_git"
```

### Template Inheritance

```yaml
# Extend base template
extends: "python-base"

# Override specific steps
steps:
  - type: render_template
    src: pyproject.toml.cli.hbs  # Override base version
    dest: pyproject.toml
  
  # Keep base steps
  - type: copy
    src: tests/
    dest: tests/
```

## 🔍 Template Debugging

### Enable Debug Logging

```bash
# Enable verbose logging
export RUST_LOG=debug

# Generate with debug output
scaffold generate --template ./python-cli --out debug-test --apply
```

### Validate Template Logic

```bash
# Test conditions
scaffold validate \
  --template ./python-cli \
  --vars use_poetry=true,include_docs=false

# Check generated structure
scaffold generate --template ./python-cli --out structure-test --apply && \
  find structure-test -type f | sort
```

### Handle Template Errors

```bash
# Check template syntax
yamllint python-cli/scaffold.yaml

# Validate Handlebars syntax
handlebars --check python-cli/files/**/*.hbs

# Test with minimal inputs
scaffold generate \
  --template ./python-cli \
  --out minimal-test \
  --vars project_name=minimal --apply
```

## 📚 Next Steps

1. **Add More Features**: Database support, configuration files, logging
2. **Multiple Frameworks**: FastAPI, Flask, Django support
3. **Advanced Testing**: Integration tests, fixtures, mocks
4. **Documentation**: Auto-generated API docs, tutorials
5. **Deployment**: Docker, Kubernetes, cloud deployment options

## 🔗 Related Resources

- [Template Authoring Guide](../templates.md) - Comprehensive template creation
- [Manifest Schema](../api/manifest.md) - Complete reference
- [Python Project](../examples/python-project.md) - Generated project usage
- [CLI Reference](../api/cli.md) - Command options
- [Getting Started](../getting-started.md) - Scaffold basics

## 🆘 Common Template Issues

### **Issue**: Variables not expanding
```handlebars
<!-- Wrong -->
{{if use_poetry}}  # Missing quotes and comparison

<!-- Correct -->
{{#if (eq use_poetry "true")}}
```

### **Issue**: Template not found
```bash
# Check template structure
find python-cli -name scaffold.yaml

# Validate manifest
scaffold validate --template ./python-cli
```

### **Issue**: Steps not executing
```yaml
# Check conditions
steps:
  - type: render_template
    src: file.hbs
    dest: file.txt
    when: "exists(var_name)"  # Make sure var_name is provided
```

### **Issue**: Handlebars syntax errors
```handlebars
<!-- Test template syntax -->
handlebars --compile python-cli/files/template.hbs

# Check preview with simple vars
echo "project_name: test" | scaffold generate --template ./python-cli --out test --apply --vars - --dry-run
```
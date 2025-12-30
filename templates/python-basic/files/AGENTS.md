# AGENTS.md

This document provides operational guidance for agentic coding agents working in this repository. It covers build, lint, test workflows, code style conventions, error handling, and how to interact with Cursor/Copilot rules when present.

--------------------------------------------------------------------------------

**Scope & intent**
- Enable precise, repeatable edits by agents while preserving project conventions.
- Emphasize surgical changes, minimal surface area, and clear rationale in commit messages where applicable.
- Respect any repository-specific AGENTS.md directives found higher up in the tree or in AGENTS.md files located at the repository root.

--------------------------------------------------------------------------------

## Build / Lint / Test Commands

The commands below are patterns to follow across common ecosystems. Adapt to the project's pyproject.toml, Makefile, or CI scripts. When available, prefer project-specific scripts (lint/test/build).

### General
- Run all sanity checks locally before proposing changes: `lint`, `build`, `test`.
- For single tests, see the per-language subsections.
- If a command runs long, provide progress feedback and, if needed, run in the background with proper monitoring.

### Python
- Build: `python -m build` or `uv build`.
- Test: `pytest` (single test: `pytest tests/test_file.py::test_name`).
- Format: `black src/ tests/` or `ruff format .`.
- Lint: `flake8 src/ tests/` or `ruff check .`.
- Type check: `mypy src/`.
- For single-test execution: Use `-k` flag: `pytest -k "test_name"`.
- For integration tests: `pytest tests/integration/`.

### General tips for single-test execution
- Prefer explicit test selectors to avoid flakiness.
- Use environment isolation (e.g., uv venv, venv, or conda) when tests touch external services.
- Capture and report test failures with full stack traces; include repro steps.

--------------------------------------------------------------------------------

## Code Style Guidelines

These guidelines help keep the codebase consistent and maintainable. When in doubt, prefer consistency with the majority of the repository.

### Imports / Dependencies
- Organize imports in the following order: standard library, third-party, local modules.
- Separate groups with a blank line.
- Avoid circular imports; prefer dependency injection where possible.
- Use absolute imports for top-level modules; use relative lazily only within the same package if needed.
- Trim unused imports; rely on the linter to enforce.

### Formatting
- Enforce a single source of truth for formatting (Black/Ruff).
- Run formatters as part of the commit hook or CI.
- Keep line length within 88 characters (Black default) or 100 characters.
- Prefer consistent quote styles (double quotes) as per project conventions.

### Types & Typing

- Python: Use type hints extensively with `typing` module.
- Use generic types for reusable components and functions.
- Leverage `Union`, `Optional`, `List`, `Dict` for complex types.
- Use dataclasses or Pydantic for structured data with validation.
- Prefer explicit return types for all functions and methods.

### Naming Conventions
- Variables: snake_case; functions: snake_case; classes: PascalCase.
- Constants: ALL_CAPS with underscores; package names: lowercase.
- Filenames: snake_case or kebab-case depending on project conventions.
- Private methods/attributes: prefix with single underscore.
- Dunder methods: double underscores (e.g., `__init__`).

### Error Handling & Logging
- Do not swallow errors; propagate with context.
- Use custom exception classes that inherit from built-in exceptions.
- Logging should be purposeful: use logging module with appropriate levels.
- In tests, assert on error conditions and messages where relevant.
- Python specific: Use try/except with specific exception types.

### Testing & Coverage
- Write unit tests for pure functions; integration tests for IO/side-effects.
- Use property-based testing where properties are known - invariants, pre/post conditions.
- Keep tests fast; mock external services; use fixtures and parametrize.
- Name tests clearly: `test_function_name_when_condition_then_expected` or `test_function_name`.
- Ensure tests run in CI with a consistent Python setup.
- Use pytest fixtures for test setup and teardown.

### Documentation & Comments
- Use docstrings for all public modules, classes, and functions (Google/NumPy style).
- Comment complex algorithms; avoid obvious statements.
- Update README/CONTRIBUTING when changes alter public behavior.
- Python specific: Use type hints in function signatures instead of comments.

### Version Control & Commits
- Small, focused commits; 1 logical change per commit.
- Write messages that explain why, not what.
- Do not commit secrets or large binary assets; use ignore rules.
- Prefer conventional commit style: `feat:` `fix:` `refactor:` `docs:`.
- Include issue/task references when available.

--------------------------------------------------------------------------------

## Virtual Environment Management

This project is configured for virtual environment usage:

### Using uv (Recommended)
```bash
# Create virtual environment
uv venv

# Activate environment
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
uv pip install -e .

# Install development dependencies
uv pip install -e ".[dev]"

# Run commands
uv run pytest
uv run black src/
```

### Using venv
```bash
# Create virtual environment
python -m venv .venv

# Activate environment
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
pip install -e .

# Install development dependencies
pip install -e ".[dev]"
```

### Environment Best Practices
- Always activate virtual environment before development
- Commit `.python-version` file for pyenv users
- Never commit `.venv/` directory (it's in .gitignore)
- Use `requirements-dev.txt` for reproducible development setup
- Test in clean virtual environments regularly

--------------------------------------------------------------------------------

## Cursor Rules

- If a repository contains Cursor rules at:
  - `.cursor/rules/` or `.cursorrules`, follow them verbatim.
- When no Cursor rules exist, default to safe, readable edits, with minimal scope.
- Avoid invasive edits that touch unrelated modules; prefer small, incremental changes.

--------------------------------------------------------------------------------

## Copilot Rules

- If a `.github/copilot-instructions.md` exists, honor its guidance.
- Provide sufficient inline context for Copilot suggestions; avoid over-automation.
- Validate Copilot-proposed edits before applying them; prefer reviewer-style verification.

--------------------------------------------------------------------------------

## Quality & Validation

- Run lint + tests locally before proposing changes.
- Ensure formatting is applied and commit diffs are minimal.
- Document non-obvious decisions in commit messages or PR descriptions.
- Python specific: Ensure `mypy` passes with strict settings before committing.

--------------------------------------------------------------------------------

## How to Use This File

- Treat this as a living contract for code agents operating in this repo.
- Update it as the project evolves; ensure it remains aligned with CI configurations.
- If you add new languages or tooling, extend the guidelines accordingly.

--------------------------------------------------------------------------------

## Python-Specific Guidelines

### Project Structure
- `src/` - Source code (src-layout for better packaging)
- `tests/` - Test files mirroring source structure
- `pyproject.toml` - Modern Python packaging and tool configuration
- `requirements-dev.txt` - Development dependencies
- `.python-version` - pyenv version specification

### Dependencies Management
- Use `pyproject.toml` for project dependencies and metadata
- Use `requirements-dev.txt` for development-only dependencies
- Prefer `uv` for faster, more reliable package management
- Pin versions in production; use ranges for development

### Type Checking
- Enable strict mypy settings for maximum type safety
- Use `typing_extensions` for newer Python features on older versions
- Prefer `Protocol` over `ABC` for duck typing
- Use `overload` for functions with multiple signatures

### Testing Strategy
- Use pytest as the testing framework
- Structure tests to mirror source code
- Use fixtures for common test setup
- Parametrize tests for multiple input scenarios
- Mock external dependencies and services

### Performance Considerations
- Profile with `cProfile` or `line_profiler` for bottlenecks
- Use appropriate data structures (lists vs deques, dicts vs sets)
- Consider async/await for I/O-bound operations
- Use `functools.lru_cache` for expensive computations

### Security Best Practices
- Review dependencies for known vulnerabilities: `pip-audit` or `safety`
- Use `secrets` module for cryptographic operations
- Validate and sanitize all external inputs
- Use parameterized queries for database operations
- Follow OWASP guidelines for web applications

This project was generated with scaffold and includes Python development environment setup, testing, linting, type checking, and CI/CD integration.
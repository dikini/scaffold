# AGENTS.md

This document provides operational guidance for agentic coding agents working in this repository. It covers build, lint, test workflows, code style conventions, error handling, and how to interact with Cursor/Copilot rules when present.

--------------------------------------------------------------------------------

**Scope & intent**
- Enable precise, repeatable edits by agents while preserving project conventions.
- Emphasize surgical changes, minimal surface area, and clear rationale in commit messages where applicable.
- Respect any repository-specific AGENTS.md directives found higher up in the tree or in AGENTS.md files located at the repository root.

--------------------------------------------------------------------------------

## Build / Lint / Test Commands

The commands below are patterns to follow across common ecosystems. Adapt to the project's package.json, Makefile, or CI scripts. When available, prefer project-specific scripts (lint/test/build).

### General
- Run all sanity checks locally before proposing changes: `lint`, `build`, `test`.
- For single tests, see the per-language subsections.
- If a command runs long, provide progress feedback and, if needed, run in the background with proper monitoring.

### Rust
- Build: `cargo build`.
- Test: `cargo test` (single test: `cargo test test_name` or integration test: `cargo test --tests`).
- Format: `rustfmt` or `cargo fmt`.
- Lint: `cargo clippy`.
- For single-test execution: Use filter arguments: `cargo test test_name -- --exact`.
- For integration tests: `cargo test --test integration_test_name`.

### General tips for single-test execution
- Prefer explicit test selectors to avoid flakiness.
- Use environment isolation (e.g., virtualenv, nvm, or Docker) when tests touch external services.
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
- Enforce a single source of truth for formatting (Prettier/Black/GoFmt/Rustfmt).
- Run formatters as part of the commit hook or CI.
- Keep line length within 80-100 characters where feasible; adjust per language standard.
- Prefer consistent quote styles (single vs double) as per project conventions.

### Types & Typing

- Rust: prefer explicit types; leverage `Result<T, E>` for errors.
- Use `Option<T>` for values that may or may not be present.
- Leverage Rust's ownership system and borrowing rules.
- Use `#[derive(Debug, Clone)]` for types that need debugging and cloning.

### Naming Conventions
- Variables: camelCase; functions: camelCase; types/classes: PascalCase.
- Constants: ALL_CAPS with underscores; package names lower_snake_case in some ecosystems.
- Filenames: kebab-case or snake_case depending on language conventions.
- Async functions: suffix with `Async` where it aids clarity (optional per project).
- Rust specific: module names in snake_case, types in PascalCase.

### Error Handling & Logging
- Do not swallow errors; propagate with context.
- Use custom error types or wrapping where helpful (e.g., `thiserror` crate).
- Logging should be purposeful: use a logger abstraction; avoid silent failures.
- In tests, assert on error conditions and messages where relevant.
- Rust specific: Use `?` operator for error propagation, `unwrap()` only when certain.

### Testing & Coverage
- Write unit tests for pure functions; integration tests for IO/side-effects.
- Use property-based testing where properties are known - invariants, pre/post conditions.
- Keep tests fast; mock external services; use fixtures.
- Name tests clearly: `TestFunctionName_WhenCondition_ThenExpected` or `test_function_name_case` style.
- Ensure tests run in CI with a consistent Rust setup.
- Rust specific: Use `#[test]` for unit tests, `#[cfg(test)]` for test modules.

### Documentation & Comments
- Use `///` for public API documentation in Rust.
- Comment complex algorithms; avoid obvious statements.
- Update README/CONTRIBUTING when changes alter public behavior.
- Rust specific: Use `cargo doc` to verify documentation builds correctly.

### Version Control & Commits
- Small, focused commits; 1 logical change per commit.
- Write messages that explain why, not just what.
- Do not commit secrets or large binary assets; use ignore rules.
- Prefer conventional commit style: `feat:` `fix:` `refactor:` `docs:`.
- Rust specific: Tag releases with semantic versioning.

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
- Rust specific: Ensure `cargo check` passes before committing.
- Run `cargo clippy` and fix all warnings before proposing changes.

--------------------------------------------------------------------------------

## How to Use This File

- Treat this as a living contract for code agents operating in this repo.
- Update it as the project evolves; ensure it remains aligned with CI configurations.
- If you add new languages or tooling, extend the guidelines accordingly.

--------------------------------------------------------------------------------

## Project-Specific Guidelines

### Rust Project Structure
- `src/main.rs` - Binary entry point
- `src/lib.rs` - Library interface
- `src/` - Source code modules
- `tests/` - Integration tests
- `Cargo.toml` - Dependencies and configuration

### Dependencies Management
- Use `cargo add` for new dependencies
- Keep dependencies updated: `cargo update`
- Review dependencies for security vulnerabilities: `cargo audit`

### Performance Considerations
- Profile using `cargo flamegraph` or similar tools
- Optimize hot paths identified through profiling
- Consider using `cargo build --release` for performance testing

### Security Best Practices
- Review dependencies for known vulnerabilities
- Use secure-by-default practices
- Handle secrets properly (don't commit them, use environment variables)

This project was generated with scaffold and includes development environment setup and CI/CD integration.
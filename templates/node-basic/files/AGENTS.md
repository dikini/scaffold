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

### Node.js/TypeScript
- Build: `npm run build`.
- Test: `npm test` (single test: `npm test -- --grep "test_name"`).
- Format: `npm run format` or `prettier --write .`.
- Lint: `npm run lint` or `eslint . --fix`.
- Watch mode: `npm run dev` or `npm run watch`.

### General tips for single-test execution
- Prefer explicit test selectors to avoid flakiness.
- Use environment isolation (e.g., nvm, or Docker) when tests touch external services.
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

- TypeScript: use interfaces for complex objects; avoid `any` unless necessary.
- Use generics for reusable components and functions.
- Leverage type inference where it improves readability.
- Use `undefined` vs `null` consistently based on project preference.
- Use enums for fixed sets of values.

### Naming Conventions
- Variables: camelCase; functions: camelCase; types/classes: PascalCase.
- Constants: ALL_CAPS with underscores; package names lower_snake_case in some ecosystems.
- Filenames: kebab-case or snake_case depending on language conventions.
- Async functions: suffix with `Async` where it aids clarity (optional per project).
- TypeScript specific: Interface names often prefixed with `I`.

### Error Handling & Logging
- Do not swallow errors; propagate with context.
- Use custom error types or wrapping where helpful.
- Logging should be purposeful: use a logger abstraction; avoid silent failures.
- In tests, assert on error conditions and messages where relevant.
- TypeScript specific: Use `try/catch` for async operations, proper error types.

### Testing & Coverage
- Write unit tests for pure functions; integration tests for IO/side-effects.
- Use property-based testing where properties are known - invariants, pre/post conditions.
- Keep tests fast; mock external services; use fixtures.
- Name tests clearly: `TestFunctionName_WhenCondition_ThenExpected` or `test_function_name_case` style.
- Ensure tests run in CI with a consistent Node/TypeScript setup.

### Documentation & Comments
- Use JSDoc / TSdoc for exported APIs.
- Comment complex algorithms; avoid obvious statements.
- Update README/CONTRIBUTING when changes alter public behavior.
- TypeScript specific: Use `/**` for JSDoc comments.

### Version Control & Commits
- Small, focused commits; 1 logical change per commit.
- Write messages that explain why, not just what.
- Do not commit secrets or large binary assets; use ignore rules.
- Prefer conventional commit style: `feat:` `fix:` `refactor:` `docs:`.
- Include task references if using task management.

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
- Node.js specific: Ensure `npm run typecheck` passes before committing.

--------------------------------------------------------------------------------

## How to Use This File

- Treat this as a living contract for code agents operating in this repo.
- Update it as the project evolves; ensure it remains aligned with CI configurations.
- If you add new languages or tooling, extend the guidelines accordingly.

--------------------------------------------------------------------------------

## Project-Specific Guidelines

### Node.js/TypeScript Project Structure
- `src/` - Source code
- `src/index.ts` - Main entry point
- `src/lib.ts` - Library exports (if applicable)
- `tests/` - Test files
- `package.json` - Dependencies and scripts
- `tsconfig.json` - TypeScript configuration

### Dependencies Management
- Use `npm install package-name` for new dependencies
- Use `npm install -D package-name` for dev dependencies
- Keep dependencies updated: `npm update`
- Review dependencies for security vulnerabilities: `npm audit`

### Performance Considerations
- Profile using built-in Node.js profiling tools or Chrome DevTools
- Optimize bundle size using webpack/rollup/vite analysis
- Consider lazy loading for large applications
- Use tree-shaking to eliminate unused code

### Security Best Practices
- Review dependencies for known vulnerabilities
- Use environment variables for secrets and configuration
- Implement proper CORS and security headers for web applications
- Use HTTPS and secure cookie practices

### TypeScript Specific
- Enable strict mode in `tsconfig.json`
- Use `strict: true` and related strict checks
- Prefer `unknown` over `any` for better type safety
- Use type guards for runtime type checking

This project was generated with scaffold and includes development environment setup, linting, testing, and CI/CD integration.
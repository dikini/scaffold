# AGENTS.md

This document provides operational guidance for agentic coding agents working in this repository. It covers build, lint, test workflows, code style conventions, error handling, task management with beads, and how to interact with Cursor/Copilot rules when present.

--------------------------------------------------------------------------------

**Scope & intent**
- Enable precise, repeatable edits by agents while preserving project conventions.
- Emphasize surgical changes, minimal surface area, and clear rationale in commit messages where applicable.
- Respect any repository-specific AGENTS.md directives found higher up in the tree or in AGENTS.md files located at the repository root.
- **Integrate with beads task management** for distributed, git-backed task tracking and dependency management.

--------------------------------------------------------------------------------

## Build / Lint / Test Commands

The commands below are patterns to follow across common ecosystems. Adapt to the project's package.json, Makefile, or CI scripts. When available, prefer project-specific scripts (lint/test/build).

### General
- Run all sanity checks locally before proposing changes: `lint`, `build`, `test`.
- For single tests, see the per-language subsections.
- If a command runs long, provide progress feedback and, if needed, run in the background with proper monitoring.

### Task Management Integration
- **Initialize beads**: `bd init` - Run once per project to set up beads task management
- **Track tasks**: Use `skill task-planner` to convert project requirements into beads tasks
- **Update progress**: Use `skill task-tracker` to manage task status and dependencies
- **Plan sprints**: Use `skill sprint-planner` for agile sprint management
- **Manage epics**: Use `skill epic-manager` for large feature organization
- **Handle dependencies**: Use `skill dependency-manager` for complex task relationships

### Rust
- Build: `cargo build`.
- Test: `cargo test` (single test: `cargo test test_name` or integration test: `cargo test --tests`).
- Format: `rustfmt` or `cargo fmt`.
- Lint: `cargo clippy`.

### Node.js/TypeScript
- Build: `npm run build`.
- Test: `npm test` (single test: `npm test -- --grep "test_name"`).
- Lint: `npm run lint` or `eslint .`.
- Format: `npm run format` or `prettier --write .`.

### Python
- Build: `python -m build`.
- Test: `pytest` (single test: `pytest tests/test_file.py::test_name`).
- Lint: `ruff check .` or `flake8 .`.
- Format: `ruff format .` or `black .`.

### Go
- Build: `go build`.
- Test: `go test ./...` (single test: `go test -run TestName ./path/to/package`).
- Lint: `golangci-lint run`.
- Format: `gofmt -s -w .`.

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
- TypeScript: use interfaces for complex objects; avoid `any` unless necessary.
- Python: use type hints; leverage dataclasses for structured data.
- Go: leverage Go's type system; prefer explicit error handling.

### Naming Conventions
- Variables: camelCase; functions: camelCase; types/classes: PascalCase.
- Constants: ALL_CAPS with underscores; package names lower_snake_case in some ecosystems.
- Filenames: kebab-case or snake_case depending on language conventions.
- Async functions: suffix with `Async` where it aids clarity (optional per project).
- Task IDs: Use beads format `bd-xxxx` for task identification.

### Error Handling & Logging
- Do not swallow errors; propagate with context.
- Use custom error types or wrapping where helpful (e.g., `cause` in Go, `Error` subclasses in JS).
- Logging should be purposeful: use a logger abstraction; avoid silent failures.
- In tests, assert on error conditions and messages where relevant.

### Testing & Coverage
- Write unit tests for pure functions; integration tests for IO/side-effects.
- use quickcheck tests, where properties are known - invariants, pre/post conditions.
- Keep tests fast; mock external services; use fixtures.
- Name tests clearly: `TestFunctionName_WhenCondition_ThenExpected` or `test_function_name_case` style.
- Ensure tests run in CI with a consistent Node/Python/Go/Rust setup.

### Documentation & Comments
- Use JSDoc / TSdoc / docstrings for exported APIs.
- Comment complex algorithms; avoid obvious statements.
- Update README/CONTRIBUTING when changes alter public behavior.

### Version Control & Commits
- Small, focused commits; 1 logical change per commit.
- Write messages that explain why, not just what.
- Include task IDs in commits: `feat(auth): add JWT support (bd-a1b2)`.
- Do not commit secrets or large binary assets; use ignore rules.
- Prefer conventional commit style: `feat:` `fix:` `refactor:` `docs:`.
- Reference beads tasks: Close tasks with `Fixes bd-a1b2` in commit messages.

--------------------------------------------------------------------------------

## Task Management with Beads

### Initialize Beads Workflow
1. **Setup**: Run `bd init` to initialize beads task management
2. **Planning**: Use `skill task-planner` to convert requirements to beads tasks
3. **Tracking**: Use `skill task-tracker` for ongoing task management
4. **Sprints**: Use `skill sprint-planner` for agile sprint planning

### Task Creation and Management
- Create tasks with bead IDs: `bd create "Implement user auth" -p 1`
- Link dependencies: `bd dep add bd-a1b2.1 bd-a1b2` (child depends on parent)
- Update status: Use task-tracker skill for automated status management
- View tasks: `bd ready` (show unblocked tasks) or `bd show bd-a1b2`

### Team Collaboration
- All tasks are git-backed and automatically synchronized
- Use hierarchical IDs for epics: `bd-a3f8` (epic) → `bd-a3f8.1` (task) → `bd-a3f8.1.1` (sub-task)
- Each agent/team member can work independently with automatic conflict prevention
- Tasks persist across branches and merge cleanly

### Integration with Development Workflow
- **Before coding**: Use beads to plan and prioritize tasks
- **During development**: Reference task IDs in commits and branches
- **After completion**: Update bead task status and capture learnings
- **During reviews**: Link PRs to corresponding bead tasks

### Best Practices
- Use consistent priority levels (P0-P3) across all tasks
- Include task dependencies in task descriptions
- Update blockers immediately when discovered
- Review and compact old tasks to maintain context efficiency
- Use stealth mode for personal experiments: `bd init --stealth`

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
- Update bead tasks when making changes related to specific tasks.
- Review task dependencies before implementing to avoid blocked work.

--------------------------------------------------------------------------------

## How to Use This File

- Treat this as a living contract for code agents operating in this repo.
- Update it as the project evolves; ensure it remains aligned with CI configurations.
- If you add new languages or tooling, extend the guidelines accordingly.
- Configure beads early in project lifecycle for optimal task management.
- Reference bead task IDs in all development work for traceability.

--------------------------------------------------------------------------------

## OpenCode Skills Integration

This project includes comprehensive opencode skills for complete product development lifecycle:

### Planning Skills
- **write-prd**: Create Product Requirements Documents
- **analyze-prd**: Analyze PRDs and generate implementation plans
- **feature-mapping**: Convert requirements to epics, features, and user stories

### Task Management Skills
- **task-planner**: Plan and organize tasks using beads
- **task-tracker**: Track, update, and manage beads tasks with automatic status updates
- **dependency-manager**: Manage complex task dependencies and resolve blockers
- **epic-manager**: Create and manage epics with hierarchical task structure
- **sprint-planner**: Plan and manage sprints with velocity tracking and capacity planning

### Technical Skills
- **tech-design-doc**: Generate technical design documents
- **project-roadmap**: Create visual project roadmaps and timeline visualizations
- **api-contract**: Generate detailed API contracts and specifications

### Usage Workflow
1. Start with planning skills to define product requirements
2. Use task management skills to create and track work with beads
3. Execute development with task reference and progress tracking
4. Iterate and refine based on feedback and completed work

This comprehensive skill set transforms this repository from just code storage into a complete product development and project management platform.

--------------------------------------------------------------------------------

## OpenCode Agents Integration

This project includes specialized opencode agents for autonomous and collaborative work:

### Available Agents
- **task-agent** - Autonomous task completion agent that finds and completes ready beads tasks
  - Mode: subagent
  - Invoked with: `@task-agent`
  - Features: Autonomous task discovery, execution, and progress reporting
  - Integration: Works with all task management skills and beads system

### Agent Usage Workflow
1. **Manual Work**: Use skills for guided task management and planning
2. **Autonomous Work**: Invoke `@task-agent` for independent task completion
3. **Collaborative Work**: Agents and humans can work in parallel on different tasks
4. **Progress Tracking**: All agents maintain audit trails and progress reporting

### Agent Invocation Examples
```bash
# Start autonomous task completion
@task-agent

# Switch between agents during work
# Tab key cycles between primary agents
# @mention invokes subagents
```

### Agent Capabilities
- **Autonomous Execution**: Agents can work independently without human supervision
- **Parallel Operation**: Multiple agents can work simultaneously on different tasks
- **Progress Communication**: Clear reporting of work status and discoveries
- **Context Awareness**: Agents understand project structure and requirements
- **Integration**: Seamless collaboration with human developers and other agents

This agent system extends the skill-based workflow into fully autonomous and collaborative development capabilities.
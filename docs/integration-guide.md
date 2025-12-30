# Brownfield Enhancement Integration Guide

This guide explains how to use the enhanced capabilities added to this brownfield project through scaffold's product development platform.

## Project Status

This project has been enhanced from a brownfield state and now includes:

- ✅ **11 OpenCode Skills** for planning and task management
- ✅ **13 Autonomous Agents** for workflow orchestration
- ✅ **Beads Task Management** with git-backed distributed tracking
- ✅ **Comprehensive Documentation** structure
- ✅ **CI/CD Automation** with quality gates
- ✅ **Legacy Preservation** in `legacy/` directory

## Quick Start

### 1. Initialize Task Management
```bash
bd init
```

### 2. Start Product Planning
```bash
@planning-phase
```

### 3. Enable Autonomous Execution
```bash
@task-agent
```

## Core Workflows

### Planning Workflow
1. **Requirements Gathering**: `@requirements-phase`
2. **Technical Design**: `@design-phase`
3. **Complete Planning**: `@planning-phase`

### Task Management Workflow
1. **Plan Tasks**: `skill task-planner`
2. **Track Progress**: `skill task-tracker`
3. **Manage Dependencies**: `skill dependency-manager`
4. **Plan Sprints**: `skill sprint-planner`

### Development Workflow
1. **Create Tasks**: `bd create "Task description" -p 1`
2. **Link Dependencies**: `bd dep add bd-child bd-parent`
3. **Update Status**: Use task-tracker skill or manual updates
4. **Reference in Commits**: Include `bd-xxxx` in commit messages

## Available Skills

### Planning Skills
- **write-prd**: Generate comprehensive Product Requirements Documents
- **analyze-prd**: Analyze PRDs and create implementation plans
- **feature-mapping**: Convert requirements to user stories and epics

### Task Management Skills
- **task-planner**: Convert features into executable beads tasks
- **task-tracker**: Update task status and manage progress
- **dependency-manager**: Resolve blockers and optimize execution order
- **epic-manager**: Create hierarchical task structures
- **sprint-planner**: Plan sprints with capacity tracking

### Technical Skills
- **tech-design-doc**: Generate architecture and design documents
- **project-roadmap**: Create visual project timelines
- **api-contract**: Generate detailed API specifications

## Agent Capabilities

### Phase Agents
- **requirements-phase**: End-to-end requirements gathering with QA
- **design-phase**: Complete technical design and planning
- **planning-phase**: Full project orchestration from idea to execution

### Task Execution Agent
- **task-agent**: Autonomous task completion with progress reporting

### Atomic Agents
- **prd-agent**: PRD generation
- **analysis-agent**: PRD analysis and planning
- **features-agent**: User story and epic creation
- **design-agent**: Technical design documents
- **roadmap-agent**: Project timelines
- **api-agent**: API specifications
- **tasks-agent**: Beads task creation
- **sprint-agent**: Sprint planning

## Beads Task Management

### Basic Commands
```bash
# Initialize (already done)
bd init

# Create tasks
bd create "Implement user authentication" -p 0

# View tasks
bd ready    # Show unblocked tasks
bd list     # Show all tasks
bd show bd-xxxx  # Show specific task

# Update status
bd update bd-xxxx --status in-progress
bd close bd-xxxx  # Mark complete

# Manage dependencies
bd dep add bd-child bd-parent
```

### Best Practices
- Use hierarchical IDs: `bd-a1b2` (epic) → `bd-a1b2.1` (task)
- Include dependencies when creating related tasks
- Update status daily during active work
- Reference task IDs in commits: `feat(auth): add JWT (bd-a1b2)`

## CI/CD Integration

The project now includes automated:
- **Testing**: Run on every push and PR
- **Linting**: Code quality checks
- **Building**: Tech stack specific builds
- **Quality Gates**: Prevent merging with issues

Configured for **Rust** projects with appropriate tooling.

## Legacy Artifacts

Original project artifacts are preserved in `legacy/`:

See `legacy/README.md` for rollback instructions if needed.

## Troubleshooting

### Skills Not Working
- Ensure `.opencode/` directory exists and contains skills
- Check that agents are properly configured
- Verify network access for agent operations

### Beads Issues
- Run `bd init` if beads system isn't initialized
- Check git status for any conflicts
- Use `bd list` to verify tasks exist

### Agent Invocation
- Use `@agent-name` syntax in opencode
- Check agent logs for error messages
- Ensure required inputs are provided

## Advanced Usage

### Custom Skill Development
Skills can be extended or new ones added to `.opencode/skill/`

### Agent Orchestration
Combine agents for complex workflows:
```bash
@requirements-phase
@design-phase
@task-agent  # Autonomous execution
```

### Integration with Existing Tools
The enhancement is designed to work alongside existing development tools and workflows.

## Getting Help

- Check `AGENTS.md` for detailed operational guidance
- Review `docs/` for comprehensive documentation
- Use `legacy/` artifacts as reference for pre-enhancement state

This enhancement provides full parity with scaffolded greenfield projects, enabling the same autonomous planning and execution capabilities.
# Beads Installation and Setup

This document provides instructions for setting up beads task management in your project.

## 📦 Installation

### Quick Install (macOS/Linux)
```bash
curl -fsSL https://raw.githubusercontent.com/steveyegge/beads/main/scripts/install.sh | bash
```

### Package Manager Installation
```bash
# npm
npm install -g @beads/bd

# Homebrew
brew install steveyegge/beads/bd

# Go (from source)
go install github.com/steveyegge/beads/cmd/bd@latest
```

## 🚀 Initial Setup

### Initialize Beads
```bash
bd init
```

This creates the `.beads/` directory structure and sets up git tracking.

### Configure for Project
```bash
# Configure beads for your project type
bd config set project.type 
bd config set team.size 3-5
bd config set sprint.duration 2weeks
```

## 📋 Basic Usage

### Create Tasks
```bash
# Create a high-priority task
bd create "Implement user authentication" -p 0

# Create an epic
bd create "User Management Epic" --epic

# Create sub-tasks
bd create "Design database schema" -p 1 --parent bd-xxxx
```

### Manage Dependencies
```bash
# Add dependency
bd dep add bd-a1b2.1 bd-a1b2

# Show ready tasks (no blockers)
bd ready

# Show task details
bd show bd-a1b2
```

### Track Progress
```bash
# Update task status
bd update bd-a1b2 --status in-progress

# Complete a task
bd close bd-a1b2

# List all tasks
bd list
```

## 🔧 Integration with OpenCode Skills

The project includes opencode skills that integrate with beads:

### Task Planning
```bash
# Convert user stories to beads tasks
skill task-planner

# Update task progress
skill task-tracker

# Plan sprints
skill sprint-planner
```

### Hierarchical Structure
Beads supports hierarchical task IDs:
- `bd-a3f8` (Epic)
- `bd-a3f8.1` (Task)
- `bd-a3f8.1.1` (Sub-task)

## 🔄 Team Workflow

### Multi-Agent Collaboration
- Each agent can work independently with hash-based IDs preventing conflicts
- Git backing ensures all changes are versioned and synchronized
- Branch-specific workflows for parallel development

### Human + Agent Workflow
- Humans handle high-level planning and review
- Agents handle task creation, updates, and progress tracking
- Seamless handoff through git-backed task system

## 🎯 Best Practices

### Task Creation
- Use descriptive titles with clear acceptance criteria
- Assign appropriate priority levels (P0-P3)
- Include dependencies and blockers in task descriptions
- Reference task IDs in commits and PRs

### Dependency Management
- Link tasks as soon as dependencies are known
- Review and update blockers regularly
- Use `bd ready` to identify immediately actionable tasks
- Plan dependencies to enable parallel work

### Progress Tracking
- Update task status daily during active work
- Close tasks with completion notes
- Review and compact old completed tasks
- Use task comments for ongoing communication

## 🔍 Advanced Features

### Stealth Mode (Personal Use)
```bash
bd init --stealth
```
Use beads locally without committing to shared repository.

### Compaction (Memory Optimization)
Beads automatically compacts old completed tasks to save context space while maintaining audit trails.

### Search and Filtering
```bash
# Search tasks
bd search "authentication"

# Filter by status
bd list --status ready

# Filter by priority
bd list --priority 0
```

This setup provides a complete task management foundation for your project, integrating with opencode skills for maximum productivity.
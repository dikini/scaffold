---
name: task-planner
description: Plan and organize tasks using beads for distributed, git-backed task management with dependency tracking
license: MIT
compatibility: opencode
metadata:
  audience: project-managers product-managers team-leads
  workflow: task-planning project-management beads-integration
  dependencies: [feature-mapping]
---

## What I do
- Plan and organize tasks using beads distributed task management system
- Create hierarchical task structures with dependency tracking
- Generate beads-ready task files with proper JSONL format
- Integrate with existing project planning outputs (PRDs, features, user stories)
- Setup beads workflow for team collaboration and agent automation

## When to use me
Use when you need to:
- Convert user stories and features into beads task structure
- Set up distributed task management for multi-agent workflows
- Create dependency-aware task planning with git backing
- Replace messy markdown TODOs with structured bead tasks
- Plan long-horizon projects with proper task tracking

## How I work
1. **Input Analysis** - Parse existing PRDs, feature breakdowns, and user stories
2. **Task Hierarchy** - Create bead-style task structure with proper IDs
3. **Dependency Mapping** - Establish task dependencies and blockers
4. **Beads Setup** - Generate `.beads/` directory structure and initial tasks
5. **Integration Configuration** - Setup beads workflow for your project type
6. **Team Workflow** - Configure beads for multi-agent and human collaboration

## What I need
- Feature breakdown or user stories (from feature-mapping skill)
- Project timeline and milestone information
- Team size and composition details
- Existing beads setup or desire to initialize new one
- Project type and complexity level

## What I produce
A complete beads task management setup containing:
- **Beads Initialization** - `.beads/` directory with proper structure
- **Task Creation** - Tasks in bead format with hierarchical IDs (bd-xxxx)
- **Dependency Graph** - Task dependencies and blocking relationships
- **Priority Assignment** - P0-P3 priority levels with proper weighting
- **Workflow Configuration** - Beads daemon and sync settings
- **Team Integration** - Multi-agent and human collaboration setup
- **Git Integration** - Branching and merging strategies for task management

## Usage Example
```
"Convert our mobile app user stories into beads task management with proper dependencies"
```

This will generate a complete beads setup with tasks, dependencies, and team workflow configuration for distributed task management.

## Beads Integration Features
- **Git-Backed Persistence** - Tasks versioned with your code
- **Conflict Prevention** - Hash-based IDs prevent merge collisions
- **Agent Optimization** - JSON output ready for AI agent consumption
- **Hierarchical Structure** - Epics → Tasks → Sub-tasks with bead IDs
- **Dependency Awareness** - Automatic blocker detection and ready task listing
- **Stealth Mode** - Option for local use without committing to main repo
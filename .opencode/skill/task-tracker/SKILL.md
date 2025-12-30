---
name: task-tracker
description: Track, update, and manage beads tasks with automatic status updates, progress monitoring, and team synchronization
license: MIT
compatibility: opencode
metadata:
  audience: developers team-leads project-managers
  workflow: task-management progress-tracking beads-maintenance
  dependencies: [task-planner]
---

## What I do
- Track and update beads tasks with automatic progress monitoring
- Manage task status changes and dependency resolution
- Provide real-time task dashboards and progress reports
- Handle beads synchronization across team members and agents
- Generate task completion workflows and validation checks

## When to use me
Use when you need to:
- Track progress on existing beads tasks
- Update task statuses as work progresses
- Monitor team task completion and dependencies
- Generate progress reports and status dashboards
- Handle task blockers and dependency resolution
- Synchronize task updates across distributed team

## How I work
1. **Task Discovery** - Scan `.beads/` directory for existing tasks
2. **Status Analysis** - Evaluate current task states and dependencies
3. **Progress Updates** - Handle task status changes and completion
4. **Dependency Resolution** - Update blockers and ready tasks
5. **Team Sync** - Manage multi-user updates and conflict resolution
6. **Reporting** - Generate progress dashboards and status reports
7. **Automation** - Handle automatic task transitions and notifications

## What I need
- Existing beads setup (.beads/ directory with tasks)
- Current task status and progress information
- Team member access and update permissions
- Project milestone and timeline context
- Any new task creation or modification requests

## What I produce
Comprehensive task tracking outputs:
- **Task Dashboard** - Real-time view of all tasks and statuses
- **Progress Reports** - Detailed progress analytics and trends
- **Dependency Updates** - Automatic blocker resolution and ready task detection
- **Team Coordination** - Multi-user update synchronization
- **Completion Workflows** - Task completion validation and transition
- **Status Summaries** - Daily/weekly progress summaries
- **Issue Detection** - Automatic identification of blocked or stuck tasks
- **Git Integration** - Task change commits and version tracking

## Usage Example
```
"Update task progress for our current sprint and handle any new blockers"
```

This will analyze current task states, update progress, handle dependencies, and provide comprehensive status reporting.

## Beads Tracking Features
- **Real-time Updates** - Immediate task status changes and propagation
- **Dependency Tracking** - Automatic detection of newly available tasks
- **Team Synchronization** - Multi-agent and human update coordination
- **Progress Analytics** - Velocity, burndown, and completion trends
- **Automated Workflows** - Status transitions and notifications
- **Conflict Resolution** - Handle simultaneous updates and merge conflicts
- **Audit Trails** - Complete change history and accountability
# Brownfield Enhancement Example

This example demonstrates how to enhance an existing project with scaffold's complete product development platform using the `brownfield-enhancement` template.

## Scenario

You have an existing Rust project that needs:
- Better project management and planning capabilities
- Task tracking and team collaboration features
- Comprehensive documentation structure
- CI/CD automation
- Autonomous development workflows

## Before Enhancement

```
my-existing-project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
├── README.md
└── .gitignore
```

## Enhancement Process

### 1. Generate Enhancement
```bash
cd my-existing-project
scaffold generate --template brownfield-enhancement --apply
```

### 2. Interactive Prompts
```
Project name? my-existing-project
Description? Enhanced existing project with full product development capabilities
Author? Your Name
License? MIT
Preserve existing artifacts in legacy/ directory? yes
What is the existing tech stack? Rust
Attempt to migrate existing task management to beads? no
How to handle existing CI workflows? replace
Include opencode skills for planning? yes
Team size? 3-5
Expected timeline (months)? 6
Initialize beads task management? yes
```

## After Enhancement

```
my-existing-project/
├── legacy/                    # Preserved originals
│   ├── README.md             # Original README
│   └── [other preserved files]
├── .opencode/                # 11 skills
│   ├── skill/
│   │   ├── task-planner/
│   │   ├── write-prd/
│   │   └── [other skills]
│   └── agent/
│       ├── planning-phase.md
│       ├── task-agent.md
│       └── [other agents]
├── .beads/                   # Task management
├── docs/                     # Documentation
│   ├── project-plan.md
│   ├── beads-setup.md
│   ├── brownfield-migration.md
│   └── integration-guide.md
├── .github/workflows/        # CI automation
│   └── ci.yml
├── config/                   # Project configuration
│   └── project-vars.yaml
├── AGENTS.md                 # Enhanced guidance
├── LICENSE                   # License file
└── scripts/                  # Migration scripts
    ├── analyze-project.sh
    └── migrate-legacy.sh
```

## Key Benefits Achieved

### 1. Complete Skill Set
- **Planning**: `write-prd`, `analyze-prd`, `feature-mapping`
- **Task Management**: `task-planner`, `task-tracker`, `dependency-manager`
- **Technical**: `tech-design-doc`, `project-roadmap`, `api-contract`

### 2. Autonomous Agents
- **Phase Agents**: `planning-phase`, `requirements-phase`, `design-phase`
- **Task Agent**: `task-agent` for independent work completion
- **Atomic Agents**: Specialized agents for specific deliverables

### 3. Task Management
- **Beads System**: Git-backed distributed task tracking
- **Hierarchical Tasks**: Epic → Task → Sub-task organization
- **Dependency Management**: Complex task relationships
- **Team Collaboration**: Conflict-free multi-agent workflows

### 4. Documentation Structure
- **Project Plan**: Comprehensive planning documentation
- **Migration Guide**: Details about the enhancement process
- **Integration Guide**: Usage instructions and workflows
- **Beads Setup**: Task management configuration

### 5. CI/CD Integration
- **GitHub Actions**: Automated testing, linting, and quality gates
- **Rust-Optimized**: Configured for Cargo workspace builds
- **Quality Assurance**: Automated checks before merges

## Workflow Adoption

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

### 4. Create and Track Tasks
```bash
bd create "Implement user authentication" -p 0
bd ready  # Show unblocked tasks
```

### 5. Commit with Task References
```bash
git commit -m "feat(auth): add JWT support (bd-a1b2)"
```

## Rollback Option

If needed, rollback to the pre-enhancement state:
```bash
# From legacy/README.md
mv legacy/README.md ./
rm -rf .opencode .beads docs .github config AGENTS.md LICENSE scripts/
```

## Advanced Usage

### Custom Skill Integration
Add project-specific skills to `.opencode/skill/`

### Agent Orchestration
Combine agents for complex workflows:
```bash
@requirements-phase
@design-phase
@task-agent
```

### Team Scaling
The enhancement supports teams of 3-5 developers with distributed workflows and autonomous execution capabilities.

## Success Metrics

After enhancement, your project will have:
- **Workflow Parity**: Same capabilities as greenfield scaffolded projects
- **Autonomous Execution**: Task-agent can complete ready work independently
- **Comprehensive Planning**: Full product development lifecycle support
- **Team Productivity**: Enhanced collaboration and task management
- **Quality Assurance**: Automated testing and quality gates

This enhancement transforms any existing project into a modern, autonomous development platform while preserving all existing work and providing easy rollback options.
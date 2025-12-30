# OpenCode Skills Implementation - Stage 2

This directory contains the complete Stage 2 implementation of opencode skills for PRD and project planning, transforming scaffold from just a project generator into a full-lifecycle product development platform.

## 🎯 Overview

The implemented skills provide a complete product planning workflow:
1. **write-prd** - Create comprehensive Product Requirements Documents
2. **analyze-prd** - Analyze PRDs and generate implementation plans
3. **feature-mapping** - Convert requirements into epics, features, and user stories
4. **tech-design-doc** - Generate technical design documents and specifications
5. **project-roadmap** - Create visual roadmaps and timeline visualizations
6. **api-contract** - Generate detailed API contracts and specifications

## 📁 Structure

```
.opencode/skill/
├── write-prd/SKILL.md          # Product Requirements Document creation
├── analyze-prd/SKILL.md        # PRD analysis and implementation planning
├── feature-mapping/SKILL.md     # Requirements to agile work items conversion
├── tech-design-doc/SKILL.md     # Technical design and architecture
├── project-roadmap/SKILL.md     # Visual roadmap and timeline creation
└── api-contract/SKILL.md        # API specification and contract generation
```

## 🚀 Usage Examples

### Basic Workflow
```bash
# Start with product idea
skill write-prd

# Analyze the completed PRD
skill analyze-prd

# Create actionable user stories
skill feature-mapping

# Design technical architecture
skill tech-design-doc

# Plan project timeline
skill project-roadmap
```

### Advanced Use Cases
```bash
# API-heavy projects can add contract generation
skill api-contract

# Teams can jump in at any stage based on their needs
skill feature-mapping  # If you already have a PRD
skill project-roadmap  # If you have requirements mapped
```

## 🔄 Skill Interdependencies

The skills are designed to work sequentially but can be used independently:

**Phase 1 (Core Skills):**
- `write-prd` → `analyze-prd` → `feature-mapping`

**Phase 2 (Technical Skills):**
- `tech-design-doc` → `project-roadmap`

**Phase 3 (Specialized Skills):**
- `api-contract` (for API-heavy projects)

## 🎨 Integration with Scaffold

These skills complement scaffold by:

1. **Planning Phase** - Provide structured product planning before code generation
2. **Template Variables** - Generated documents can inform scaffold template variables
3. **Documentation Integration** - Created docs integrate with scaffold's documentation structure
4. **CI/CD Alignment** - Skills support scaffold's existing CI/CD and testing patterns

## 📊 Key Features

### Modern Product Management
- Industry-standard PRD templates
- Agile-friendly user story generation
- Stakeholder-aligned documentation
- Risk assessment and mitigation planning

### Technical Excellence
- Architecture-first design approach
- API-first development support
- Security and performance planning
- Scalability and maintainability focus

### Visual Communication
- Multiple roadmap formats (Gantt, timeline, Kanban)
- Stakeholder-appropriate visualizations
- Progress tracking and milestone reporting
- Dependency visualization

## 🛠️ Configuration

The `opencode.json` file configures skill permissions:
```json
{
  "permission": {
    "skill": {
      "write-prd": "allow",
      "analyze-prd": "allow", 
      "feature-mapping": "allow",
      "tech-design-doc": "allow",
      "project-roadmap": "allow",
      "api-contract": "allow"
    }
  }
}
```

## 🎯 Target Users

Each skill is designed for specific roles:

- **Product Managers**: `write-prd`, `analyze-prd`, `feature-mapping`, `project-roadmap`
- **Engineering Leads**: `analyze-prd`, `tech-design-doc`, `api-contract`
- **Developers**: `feature-mapping`, `tech-design-doc`, `api-contract`
- **Project Managers**: `analyze-prd`, `project-roadmap`, `feature-mapping`
- **Executives**: `analyze-prd`, `project-roadmap`

## 📈 Success Metrics

- Reduced planning time for new projects
- Improved requirement quality and completeness
- Better alignment between business and technical teams
- Faster onboarding for new team members
- Consistent documentation across projects

## 🔮 Future Enhancements

Potential future skills could include:
- **market-research** - Competitive analysis and market validation
- **user-testing** - Test plan creation and result analysis
- **compliance-docs** - Regulatory compliance documentation
- **performance-budget** - Performance specification and monitoring
- **security-audit** - Security requirements and testing plans

## 🤝 Contributing

These skills follow the opencode skill standards:
- Proper YAML frontmatter with name, description, metadata
- Consistent structure: What I do, When to use me, How I work, etc.
- Clear dependencies and integration points
- Actionable examples and usage patterns

This implementation transforms scaffold into a comprehensive product development platform that guides users from initial idea to technical implementation while maintaining the simplicity and power that makes scaffold effective.
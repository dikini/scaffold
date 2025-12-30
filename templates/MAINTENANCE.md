# Template Maintenance Guide

This document outlines the maintenance procedures for scaffold templates, ensuring parity and compatibility between `product-planning` and `brownfield-enhancement` templates.

## Shared Components Architecture

### Directory Structure
```
templates/
├── _shared/                    # Common components
│   ├── .opencode/            # Skills and agents
│   ├── docs/                 # Documentation templates
│   ├── .github/workflows/    # CI templates
│   ├── AGENTS.md             # Base agent guidance
│   └── version.json          # Version tracking
├── product-planning/         # Greenfield template
└── brownfield-enhancement/   # Brownfield template
```

### Update Procedures

#### When Updating Shared Components

1. **Modify Shared Files**:
   ```bash
   # Update skills, agents, docs, or CI in _shared/
   vim templates/_shared/.opencode/skill/task-planner/SKILL.md
   ```

2. **Update Version**:
   ```bash
   # Increment version and update changelog
   vim templates/_shared/version.json
   ```

3. **Test Both Templates**:
   ```bash
   # Test product-planning template
   scaffold generate --template product-planning --out test-greenfield --dry-run

   # Test brownfield-enhancement template
   scaffold generate --template brownfield-enhancement --out test-brownfield --dry-run
   ```

4. **Commit Changes**:
   ```bash
   git add templates/_shared/
   git commit -m "feat(shared): update task-planner skill

   - Enhanced dependency tracking
   - Updated version to 0.1.1
   - Tested compatibility with both templates"
   ```

#### When Adding New Skills/Agents

1. **Add to Shared**:
   ```bash
   mkdir templates/_shared/.opencode/skill/new-skill
   # Create SKILL.md and implementation
   ```

2. **Update Templates** (if needed):
   - Add to `enabled_skills` in config files
   - Update AGENTS.md documentation
   - Test integration

3. **Update Version and Test**

#### When Modifying Template-Specific Logic

1. **Update Individual Template**:
   ```bash
   # For brownfield-specific changes
   vim templates/brownfield-enhancement/scaffold.yaml
   ```

2. **Test Template Isolation**:
   - Ensure changes don't affect shared components
   - Test template-specific features
   - Verify no conflicts with shared usage

## Compatibility Testing

### Automated Testing
Run the test suite for both templates:
```bash
# Test all templates
scaffold test --template product-planning
scaffold test --template brownfield-enhancement
```

### Cross-Template Validation
- Generate projects with both templates
- Compare skill/agent functionality
- Verify documentation consistency
- Check CI workflow compatibility

### Version Compatibility
- Ensure shared version.json is consistent
- Check that templates can use updated shared components
- Maintain backward compatibility where possible

## Release Coordination

### Version Numbering
- Shared components: `0.1.x` (increment patch for updates)
- Individual templates: `0.1.x` (independent versioning)
- Major versions: `1.x.x` (breaking changes)

### Release Checklist
- [ ] Update shared version.json
- [ ] Test both templates thoroughly
- [ ] Update this maintenance guide if needed
- [ ] Commit with clear changelog
- [ ] Tag release if appropriate

## Troubleshooting

### Common Issues

**Skills not loading**: Check that shared .opencode/ is properly referenced in scaffold.yaml

**Documentation mismatches**: Ensure template-specific docs don't conflict with shared docs

**CI failures**: Verify workflow compatibility across templates

**Version conflicts**: Check version.json and update accordingly

### Recovery Procedures

**Rollback shared changes**:
```bash
git revert <commit-hash>
scaffold generate --template product-planning --out recovery-test
```

**Isolate template issues**:
- Test templates individually
- Compare with known working versions
- Check scaffold.yaml syntax

## Contact

For template maintenance questions or issues, refer to the main scaffold project documentation and issue tracking.
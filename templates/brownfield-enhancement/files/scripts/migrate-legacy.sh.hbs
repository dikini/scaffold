#!/bin/bash
# migrate-legacy.sh - Preserve existing artifacts in legacy/ directory

echo "📁 Creating legacy preservation..."

# Create legacy directory
mkdir -p legacy

# Move existing artifacts
if [ -f "AGENTS.md" ]; then
    echo "   ✅ Preserving AGENTS.md to legacy/"
    mv AGENTS.md legacy/
fi

if [ -d ".beads" ]; then
    echo "   ✅ Preserving .beads to legacy/.beads"
    mv .beads legacy/
fi

if [ -d ".opencode" ]; then
    echo "   ✅ Preserving .opencode to legacy/.opencode"
    mv .opencode legacy/
fi

if [ -d "docs" ]; then
    echo "   ✅ Preserving docs to legacy/docs"
    mv docs legacy/
fi

if [ -d ".github" ]; then
    echo "   ✅ Preserving .github to legacy/.github"
    mv .github legacy/
fi

# Create README for legacy
cat > legacy/README.md << 'EOF'
# Legacy Artifacts

This directory contains the original artifacts from before the brownfield enhancement with scaffold's product development platform.

## Preserved Items

- **AGENTS.md**: Original agent guidance file
- **.beads/**: Existing beads task management (if any)
- **.opencode/**: Existing opencode skills and agents (if any)
- **docs/**: Existing documentation
- **.github/**: Existing CI workflows and configurations

## Rollback Instructions

To rollback to the pre-enhancement state:

1. Remove the new scaffolded files:
   ```bash
   rm -rf .opencode .beads docs .github AGENTS.md config/
   ```

2. Restore from legacy:
   ```bash
   mv legacy/AGENTS.md ./
   mv legacy/.beads ./ 2>/dev/null || true
   mv legacy/.opencode ./ 2>/dev/null || true
   mv legacy/docs ./ 2>/dev/null || true
   mv legacy/.github ./ 2>/dev/null || true
   ```

3. Remove the legacy directory:
   ```bash
   rm -rf legacy/
   ```

## Integration Notes

The scaffolded enhancement provides:
- Full product development platform with 11 skills and 13 agents
- Git-backed beads task management
- Comprehensive documentation structure
- CI/CD automation

Legacy artifacts are preserved for reference but may need updates to work with the new system.
EOF

echo "   ✅ Created legacy/README.md with rollback instructions"

echo "🛡️  Legacy preservation complete"
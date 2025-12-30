# Complete 0-to-MVP Workflow: Rust CLI Tool

This guide demonstrates the complete workflow for building a Rust CLI tool from concept to working MVP using the scaffold system with atomic + phase agents.

## 🎯 Overview

**Project**: rust-cli-example - A command-line tool for text file processing
**Timeline**: 8 weeks (2 months)
**Team**: 1 developer
**Technology**: Rust with clap, regex, and other crates

## 📋 Phase 1: Planning (Weeks 1-2)

### Step 1.1: Initialize Product Planning Project

```bash
# Generate comprehensive planning project
scaffold generate --template product-planning \
  --out rust-cli-example \
  --vars '{"project_name":"rust-cli-example","description":"A command-line tool for processing text files with various transformations","author":"Your Name","license":"MIT","include_skills":"true","project_type":"other","team_size":"1-2","timeline_months":"2","initialize_beads":"true"}' \
  --apply
```

**Output**: Complete planning environment with:
- OpenCode skills for planning
- Beads task management
- Documentation templates
- Agent infrastructure

### Step 1.2: Execute Planning Phase Agents

```bash
cd rust-cli-example

# Run the complete planning orchestration
@planning-phase
```

**What happens**:
1. `@requirements-phase` invokes atomic agents:
   - `@prd-agent` → generates `docs/prd.md`
   - `@analysis-agent` → generates `docs/prd-analysis.md`
   - `@features-agent` → generates `docs/feature-backlog.md`
2. `@design-phase` invokes atomic agents:
   - `@design-agent` → generates `docs/technical-design.md`
   - `@roadmap-agent` → generates `docs/project-roadmap.md`
   - `@api-agent` → generates `docs/api-contract.md` (if applicable)
3. `@tasks-agent` + `@sprint-agent` create executable beads tasks

**Quality Assurance**: Each phase includes automated QA with 80%+ pass threshold, collaborative error recovery.

### Step 1.3: Review Planning Outputs

```bash
# Check generated documentation
ls docs/
# prd.md, prd-analysis.md, feature-backlog.md, technical-design.md, project-roadmap.md, sprint-plan.md, planning-complete.md

# Review planning completion report
cat docs/planning-complete.md
```

**Planning Deliverables**:
- ✅ Product Requirements Document (PRD)
- ✅ Technical feasibility analysis
- ✅ Agile feature backlog (85 points)
- ✅ System architecture design
- ✅ 8-week project roadmap
- ✅ Sprint execution plan
- ✅ Quality assurance reports

## 🏗️ Phase 2: Implementation (Weeks 3-6)

### Step 2.1: Generate Development Project

```bash
# Generate Rust project from planning outputs
scaffold generate --template rust-basic --out rust-cli-implementation --apply

# Copy planning documentation for integration
cp -r ../rust-cli-example/docs rust-cli-implementation/
```

### Step 2.2: Sprint 1 - Foundation (Week 3)

**Goal**: Basic CLI structure and core transformations

```bash
cd rust-cli-implementation

# Update Cargo.toml with dependencies from technical design
# clap, regex, anyhow, thiserror, indicatif

# Implement basic CLI structure
cargo build
./target/debug/rust-cli-example --help
```

**Sprint 1 Deliverables**:
- ✅ CLI argument parsing with clap
- ✅ File I/O (stdin/files)
- ✅ Uppercase transformation
- ✅ Lowercase transformation
- ✅ Unit tests for core functionality

### Step 2.3: Sprint 2 - Core Features (Week 4)

**Goal**: Complete essential text processing capabilities

```bash
# Implement deduplication
echo -e "line1\nline2\nline1" | ./target/debug/rust-cli-example dedup
# Output: line1\nline2

# Add file output support
echo "hello world" | ./target/debug/rust-cli-example upper -o output.txt
cat output.txt  # HELLO WORLD
```

**Sprint 2 Deliverables**:
- ✅ Line deduplication
- ✅ File output support (-o flag)
- ✅ Error handling and validation
- ✅ Integration tests

### Step 2.4: Sprint 3 - Advanced Features (Week 5)

**Goal**: Regex and advanced transformations

```bash
# Implement regex replacement
echo "Hello 123 World" | ./target/debug/rust-cli-example replace "\d+" "XXX"
# Output: Hello XXX World

# Add progress indicators for large files
dd if=/dev/zero bs=1M count=10 | tr '\0' 'x' > large_file.txt
./target/debug/rust-cli-example upper large_file.txt  # Shows progress bar
```

**Sprint 3 Deliverables**:
- ✅ Regex text replacement
- ✅ Line numbering
- ✅ Progress indicators
- ✅ Performance optimization

### Step 2.5: Sprint 4 - Polish & Testing (Week 6)

**Goal**: Production-ready software

```bash
# Run comprehensive tests
cargo test
cargo clippy
cargo fmt --check

# Performance benchmarking
time ./target/debug/rust-cli-example upper large_file.txt > /dev/null

# Generate documentation
cargo doc --open
```

**Sprint 4 Deliverables**:
- ✅ 90%+ test coverage
- ✅ Performance within targets (<30s for 100MB)
- ✅ Complete documentation
- ✅ Cross-platform compatibility
- ✅ Ready for release

## 🚀 Phase 3: Release & Launch (Weeks 7-8)

### Step 3.1: Final Testing & Packaging

```bash
# Build release version
cargo build --release

# Test release binary
./target/release/rust-cli-example --version

# Package for distribution
# Create release archives for different platforms
```

### Step 3.2: Documentation & Community

```bash
# Update README with comprehensive usage examples
# Create man pages and shell completions
# Set up GitHub repository and documentation
```

### Step 3.3: Release & Distribution

```bash
# Tag release
git tag v1.0.0

# Publish to package managers
cargo publish  # Publish to crates.io

# Create GitHub release with binaries
# Announce on relevant communities
```

## 🎯 Success Metrics Achieved

### Technical Success ✅
- **Performance**: Process 100MB file in <30 seconds ✅
- **Memory**: Peak usage <100MB ✅
- **Compatibility**: Works on Linux, macOS, Windows ✅
- **Test Coverage**: >90% ✅

### Business Success ✅
- **Downloads**: Ready for community adoption
- **User Satisfaction**: Intuitive CLI design
- **Maintainability**: Clean, documented code
- **Extensibility**: Modular architecture for future features

## 🛠️ Key Tools & Technologies

### Planning Phase
- **Scaffold**: Project generation and agent orchestration
- **OpenCode Agents**: Atomic agents (prd-agent, design-agent, etc.) + phase agents (requirements-phase, design-phase)
- **Beads**: Distributed task management
- **Markdown**: Documentation format

### Development Phase
- **Rust**: Systems programming language
- **Clap**: CLI argument parsing
- **Regex**: Pattern matching and replacement
- **Anyhow/Thiserror**: Error handling
- **Indicatif**: Progress indicators

### Quality Assurance
- **Cargo Test**: Unit and integration testing
- **Clippy**: Code linting
- **Rustfmt**: Code formatting
- **GitHub Actions**: CI/CD pipeline

## 📈 Lessons Learned

### Planning Phase Insights
- **Agent Orchestration**: Phase agents provide excellent coordination with automated QA
- **Documentation Quality**: Comprehensive planning reduces implementation uncertainty
- **Iterative Refinement**: Agent collaboration improves output quality

### Development Phase Insights
- **Incremental Delivery**: Sprint-based approach maintains momentum
- **Quality Gates**: Automated testing catches issues early
- **Performance Focus**: Early benchmarking prevents optimization crises

### Overall Project Success
- **Tooling Investment**: Scaffold + agents reduced planning time by 60%
- **Quality Assurance**: Automated QA prevented major rework
- **Documentation**: Living documentation improved team communication

## 🔄 Continuous Improvement

### Future Enhancements
- Add regex pattern validation
- Implement streaming for files >1GB
- Add CSV and JSON processing
- Create plugin architecture

### Process Improvements
- Enhanced agent collaboration patterns
- Automated performance regression testing
- Improved error recovery mechanisms

---

**Total Timeline**: 8 weeks
**Lines of Code**: ~500 (core functionality)
**Test Coverage**: 92%
**Performance**: 95 MB/s processing speed
**Status**: ✅ MVP Complete - Ready for Production Use

This workflow demonstrates how scaffold's atomic + phase agent architecture transforms planning into executable development, delivering high-quality software with comprehensive documentation and automated quality assurance.
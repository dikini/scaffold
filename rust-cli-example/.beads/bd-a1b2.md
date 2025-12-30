# bd-a1b2: Read from Standard Input
**Status**: ready
**Priority**: P0
**Assignee**: unassigned
**Estimate**: 3 points

**Description**:
As a developer using the tool in pipelines, I want to read text from stdin so that I can use it in shell pipelines.

**Acceptance Criteria**:
- Tool accepts input from stdin when no file specified
- Processes stdin input identically to file input
- Handles large stdin streams without memory issues
- Returns appropriate exit codes

**Dependencies**: None

**Created**: 2024-12-30
**Updated**: 2024-12-30
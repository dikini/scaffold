# API Contract Specification

**Version**: 1.0.0  
**Date**: December 2024  
**API Type**: Command-Line Interface  
**Contract Type**: Stable  

## Overview

This document specifies the API contract for Scaffold's command-line interface, including command structure, input/output formats, error handling, and integration patterns.

## API Endpoints (Commands)

### 1. List Templates

**Endpoint**: `scaffold list-templates`  
**Method**: GET  
**Stability**: Stable  

#### Request
```bash
scaffold list-templates [--json]
```

#### Parameters
- `--json` (optional): Output in JSON format instead of human-readable text

#### Response (Text Format)
```
Available templates:

  rust-basic (0.1.0)
    Rust Basic Project
    A basic Rust project with CI, linting, and tests
    Language: rust

  [additional templates...]
```

#### Response (JSON Format)
```json
[
  {
    "id": "rust-basic",
    "name": "Rust Basic Project",
    "description": "A basic Rust project with CI, linting, and tests",
    "version": "0.1.0",
    "language": "rust"
  }
]
```

#### Error Responses
- **Exit Code 1**: Unable to read templates directory
- **Exit Code 2**: Invalid template manifest format

### 2. Validate Template

**Endpoint**: `scaffold validate`  
**Method**: POST (validation)  
**Stability**: Stable  

#### Request
```bash
scaffold validate --template <id|path> [--vars <file>] [--json]
```

#### Parameters
- `--template` (required): Template ID or file path
- `--vars` (optional): YAML file with template variables
- `--json` (optional): Output in JSON format

#### Response (Success)
```
Template 'rust-basic' is valid
```

#### Response (JSON Format)
```json
{
  "template": "rust-basic",
  "valid": true,
  "version": "0.1.0",
  "warnings": []
}
```

#### Error Response (Validation Failed)
```
Template 'invalid-template' validation failed:
  error: manifest error: missing required field `id`
```

#### Error Response (JSON Format)
```json
{
  "template": "invalid-template",
  "valid": false,
  "errors": [
    {
      "field": "id",
      "message": "missing required field"
    }
  ]
}
```

### 3. Generate Project

**Endpoint**: `scaffold generate`  
**Method**: POST (generation)  
**Stability**: Stable  

#### Request
```bash
scaffold generate \
  --template <id|path> \
  --out <directory> \
  [--vars <file>] \
  [--dry-run] \
  [--apply] \
  [--allow-scripts] \
  [--commit] \
  [--json]
```

#### Parameters
- `--template` (required): Template ID or file path
- `--out` (required): Output directory path
- `--vars` (optional): YAML file with template variables
- `--dry-run` (optional): Preview changes without applying
- `--apply` (optional): Apply changes to disk (default: false)
- `--allow-scripts` (optional): Allow script execution during generation
- `--commit` (optional): Automatically commit generated project
- `--json` (optional): Output in JSON format

#### Interactive Variables
When variables are not provided via `--vars`, the CLI prompts interactively:

```
Project name? my-awesome-project
Description? A new project
Author? Developer Name
License? MIT
```

#### Response (Success)
```
Project generated successfully at /path/to/project

Files created:
  [created] Cargo.toml
  [created] src/main.rs
  [created] .gitignore
  [created] README.md
  [created] .github/workflows/ci.yml
```

#### Response (JSON Format)
```json
{
  "success": true,
  "output_path": "/path/to/project",
  "files_created": [
    "Cargo.toml",
    "src/main.rs",
    ".gitignore",
    "README.md",
    ".github/workflows/ci.yml"
  ],
  "commit_hash": "a1b2c3d4..."
}
```

#### Error Response
```
Error: Template 'nonexistent' not found
Available templates: rust-basic, node-basic, python-basic
```

### 4. Initialize Project

**Endpoint**: `scaffold init`  
**Method**: POST (interactive generation)  
**Stability**: Stable  

#### Request
```bash
scaffold init [--template <id>] [--interactive] [--out <directory>] [--json]
```

#### Parameters
- `--template` (optional): Preferred template ID
- `--interactive` (optional): Force interactive mode
- `--out` (optional): Output directory (default: current)
- `--json` (optional): Output in JSON format

#### Interactive Flow
```
Welcome to Scaffold!
? Choose a template: (Use arrow keys)
❯ rust-basic
  node-basic
  python-basic
  product-planning

? Project name: my-project
? Description: My awesome project
? Author: Developer Name
? License: MIT

Generating project...
✅ Project created successfully!
```

#### Response (JSON Format)
```json
{
  "success": true,
  "template_used": "rust-basic",
  "output_path": "/current/directory",
  "variables": {
    "project_name": "my-project",
    "description": "My awesome project",
    "author": "Developer Name",
    "license": "MIT"
  }
}
```

### 5. Test Project

**Endpoint**: `scaffold test`  
**Method**: POST (testing)  
**Stability**: Stable  

#### Request
```bash
scaffold test --target <directory> [--json]
```

#### Parameters
- `--target` (required): Directory containing generated project
- `--json` (optional): Output in JSON format

#### Response (Success)
```
Running tests for project at /path/to/project...
✅ Template validation passed
✅ File structure correct
✅ Scripts executable (where applicable)
✅ Dependencies resolvable
✅ CI configuration valid

All tests passed!
```

#### Response (JSON Format)
```json
{
  "success": true,
  "target": "/path/to/project",
  "tests_run": [
    "template_validation",
    "file_structure",
    "script_execution",
    "dependency_resolution",
    "ci_validation"
  ],
  "results": {
    "passed": 5,
    "failed": 0,
    "warnings": 0
  }
}
```

## Data Models

### Template Information
```typescript
interface Template {
  id: string;
  name: string;
  description: string;
  version: string;
  language: string;
}
```

### Generation Result
```typescript
interface GenerationResult {
  success: boolean;
  output_path: string;
  files_created: string[];
  commit_hash?: string;
  errors?: string[];
}
```

### Validation Result
```typescript
interface ValidationResult {
  template: string;
  valid: boolean;
  version: string;
  errors?: ValidationError[];
  warnings?: string[];
}

interface ValidationError {
  field: string;
  message: string;
}
```

## Error Handling

### HTTP-Style Status Codes (CLI Exit Codes)
- **0**: Success
- **1**: General error (invalid arguments, file not found)
- **2**: Template validation error
- **3**: Generation error (file conflicts, permission issues)
- **4**: Network policy violation
- **5**: Script execution error

### Error Response Format
```
Error: [Error Type]: [Detailed message]
[Suggestions for resolution]
```

### Common Error Scenarios
1. **Template Not Found**
   ```
   Error: Template 'unknown-template' not found
   Available templates: rust-basic, node-basic, python-basic
   ```

2. **Permission Denied**
   ```
   Error: Permission denied writing to /protected/path
   Try running with appropriate permissions or choose a different output directory
   ```

3. **Network Policy Violation**
   ```
   Error: Network policy violation: command blocked by policy: curl
   Use --allow-scripts to enable network access, or modify template network policy
   ```

## Authentication & Authorization

**Current Status**: Not required (CLI tool)  
**Future Consideration**: API key authentication for enterprise deployments

## Rate Limiting

**Current Status**: No rate limiting  
**Future Consideration**: Per-user/hour limits for API endpoints

## Versioning

### API Versioning
- **Current**: v1 (implied in CLI commands)
- **Future**: Explicit versioning with `--api-version` flag

### Compatibility
- **Backward Compatible**: All v1.x releases maintain CLI compatibility
- **Breaking Changes**: Only in major version releases (v2.x)

## Integration Patterns

### CI/CD Integration
```yaml
# GitHub Actions
- name: Generate project
  run: scaffold generate --template rust-basic --out . --apply --json
```

### Script Integration
```bash
# Generate and capture result
result=$(scaffold generate --template rust-basic --out ./project --apply --json)
success=$(echo $result | jq -r '.success')
```

### Error Handling in Scripts
```bash
if ! scaffold generate --template rust-basic --out ./project --apply; then
  echo "Generation failed, checking logs..."
  exit 1
fi
```

## Monitoring & Observability

### Logging
- **CLI Output**: Human-readable progress and error messages
- **JSON Mode**: Structured data for programmatic consumption
- **Verbose Mode**: Detailed operation logging (future feature)

### Metrics
- **Generation Time**: Time to complete project generation
- **Success Rate**: Percentage of successful generations
- **Template Usage**: Most popular templates and languages

## Future API Extensions

### REST API (Planned v0.3.0)
```
POST /api/v1/generate
GET  /api/v1/templates
POST /api/v1/validate
GET  /api/v1/templates/{id}
```

### WebSocket Support (Planned v1.0.0)
- Real-time generation progress
- Interactive variable prompting
- Live validation feedback

## Support

### Documentation
- **CLI Help**: `scaffold --help` and `scaffold <command> --help`
- **Online Docs**: https://scaffold.dev/docs/api/cli
- **Examples**: https://scaffold.dev/docs/examples

### Issue Reporting
- **Bug Reports**: GitHub Issues with `--json` output
- **Feature Requests**: GitHub Discussions
- **Security Issues**: security@scaffold.dev

## Changelog

### v1.0.0 (Current)
- Initial stable API contract
- JSON output format standardization
- Comprehensive error handling
- Interactive mode support

### Future Versions
- **v1.1.0**: Enhanced error messages and suggestions
- **v1.2.0**: Progress reporting and cancellation support
- **v2.0.0**: REST API introduction (breaking change)
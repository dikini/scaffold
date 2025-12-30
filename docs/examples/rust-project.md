# Rust Project Example

Complete example of generating a Rust web application using the rust-basic template.

## 🎯 Goal

Create a Rust web application with:
- Axum web framework
- Async support
- Tokio runtime
- SQLx for database
- GitHub Actions CI
- Proper project structure

## 📋 Prerequisites

- Scaffold installed
- Rust 1.70+ for generated project
- Text editor

## 🚀 Quick Example

### Step 1: Create Variables File

```yaml
# web-app-vars.yaml
project_name: "rust-web-scraper"
description: "A web scraper that extracts data from websites"
author: "Your Name <you@example.com>"
license: "MIT"
use_async: true
```

### Step 2: Generate Project

```bash
# Preview first
scaffold generate \
  --template rust-basic \
  --out rust-web-scraper \
  --vars web-app-vars.yaml \
  --dry-run

# Apply changes
scaffold generate \
  --template rust-basic \
  --out rust-web-scraper \
  --vars web-app-vars.yaml \
  --apply

# Auto-commit git repo
scaffold generate \
  --template rust-basic \
  --out rust-web-scraper \
  --vars web-app-vars.yaml \
  --apply \
  --commit
```

### Step 3: Customize Generated Project

```bash
cd rust-web-scraper

# Add dependencies to Cargo.toml
cat >> Cargo.toml << 'EOF'

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
scraper = "0.17"
regex = "1.0"
EOF

# Update main.rs for web server
cat > src/main.rs << 'EOF'
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ScrapeResult {
    url: String,
    title: String,
    data: Vec<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/scrape", get(scrape))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(app, listener).await.unwrap();
}

async fn root() -> &'static str {
    "Rust Web Scraper API\nUse POST /scrape with {\"url\": \"https://example.com\"}"
}

async fn health() -> Json<HashMap<String, String>> {
    Json(HashMap::from([("status".to_string(), "healthy".to_string())]))
}

async fn scrape(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ScrapeResult>, StatusCode> {
    let url = params.get("url").unwrap_or(&"".to_string());
    
    if url.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // In a real implementation, you would:
    // 1. Fetch the URL
    // 2. Parse HTML
    // 3. Extract data
    // 4. Return structured result
    
    let result = ScrapeResult {
        url: url.clone(),
        title: "Example Page Title".to_string(),
        data: vec![
            "Extracted data 1".to_string(),
            "Extracted data 2".to_string(),
        ],
    };

    Ok(Json(result))
}
EOF

# Update lib.rs with utilities
cat > src/lib.rs << 'EOF'
//! Web scraping utilities.

use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;

pub async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    Ok(html)
}

pub fn extract_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("title").unwrap();
    
    document
        .select(&selector)
        .next()
        .map(|element| element.inner_html())
}

pub fn extract_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();
    
    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|href| href.to_string())
        .collect()
}

pub fn parse_json_structure(html: &str) -> Value {
    // Custom parsing logic here
    serde_json::json!({
        "title": extract_title(html),
        "links": extract_links(html),
        "word_count": html.split_whitespace().count()
    })
}
EOF
```

### Step 4: Test the Project

```bash
# Build project
cargo build

# Run tests
cargo test

# Start development server
cargo run

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## 🔧 Advanced Configuration

### Multiple Variables Files

```bash
# Base configuration
cat > base-vars.yaml << 'EOF'
project_name: "{{project_name}}"
description: "{{description}}"
author: "Your Name <you@example.com>"
license: "MIT"
use_async: true
EOF

# Project-specific overrides
cat > scraper-vars.yaml << 'EOF'
project_name: "advanced-scraper"
description: "Advanced web scraper with features"
use_async: true
EOF

# Generate with merged variables
scaffold generate \
  --template rust-basic \
  --out advanced-scraper \
  --vars scraper-vars.yaml \
  --apply
```

### Conditional Generation

```yaml
# Conditional variables based on environment
# prod-vars.yaml
project_name: "production-api"
description: "Production API server"
license: "MIT"
use_async: true

# dev-vars.yaml  
project_name: "dev-api"
description: "Development API server"
license: "MIT"
use_async: false
```

```bash
# Environment-based generation
if [ "$ENV" = "production" ]; then
  scaffold generate --template rust-basic --out prod-api --vars prod-vars.yaml --apply
else
  scaffold generate --template rust-basic --out dev-api --vars dev-vars.yaml --apply
fi
```

## 📱 Generated Project Structure

```
rust-web-scraper/
├── .git/
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── rustfmt.toml
├── clippy.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── main.rs.hbs.backup
├── target/
└── .github/
    └── workflows/
        └── ci.yml
```

## 🚀 CI/CD Integration

### GitHub Actions Workflow

The generated `.github/workflows/ci.yml` includes:

```yaml
name: CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Cache cargo
        uses: actions/cache@v4
      - name: Check formatting
        run: cargo fmt -- --check
      - name: Clippy
        run: cargo clippy -- -D warnings
      - name: Build
        run: cargo build --verbose
      - name: Test
        run: cargo test --verbose
```

### Local Testing

```bash
# Test CI locally
act -j ubuntu-latest

# Or use GitHub CLI
gh workflow run "CI"
```

## 🔍 Troubleshooting

### Build Errors

```bash
# Check Rust version
rustc --version

# Update dependencies
cargo update

# Clean and rebuild
cargo clean && cargo build
```

### Test Failures

```bash
# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Debug tests
RUST_BACKTRACE=1 cargo test
```

### Permission Issues

```bash
# Check file permissions
ls -la

# Fix permissions
chmod +x scripts/*.sh
```

## 📚 Next Steps

1. **Database Integration**: Add SQLx migrations
2. **Authentication**: Add JWT middleware
3. **Configuration**: Add config file support
4. **Docker**: Add Dockerfile and docker-compose
5. **API Documentation**: Add OpenAPI/Swagger docs

## 🔗 Related Examples

- [Node.js Project](node-project.md) - Similar setup for Node.js
- [Custom Template](custom-template.md) - Create your own template
- [CI Integration](../getting-started.md#adding-ci-to-existing-project) - Add CI to existing projects

## 🆘 Common Issues

### **Issue**: Template variables not expanding
```bash
# Check variables file syntax
scaffold validate --template rust-basic --vars your-vars.yaml

# Verify variable names
grep -E "{{[^}]+}}" your-vars.yaml
```

### **Issue**: Build fails after generation
```bash
# Check Cargo.toml syntax
cargo check

# Update dependencies
cargo update

# Check Rust toolchain
rustup show
```

### **Issue**: CI tests failing locally
```bash
# Check workflow syntax
yamllint .github/workflows/ci.yml

# Run same environment as CI
docker run --rm -v $(pwd):/app -w /app rust:latest cargo test
```
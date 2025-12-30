# Node.js Project Example

Complete example of generating a Node.js TypeScript application using the node-basic template.

## 🎯 Goal

Create a Node.js web API with:
- Express.js framework
- TypeScript support
- ESLint and Prettier
- Jest testing
- GitHub Actions CI
- Modern project structure

## 📋 Prerequisites

- Scaffold installed
- Node.js 18+ for generated project
- npm or yarn package manager

## 🚀 Quick Example

### Step 1: Create Variables File

```yaml
# api-vars.yaml
project_name: "node-express-api"
description: "A Node.js REST API with Express and TypeScript"
author: "Your Name <you@example.com>"
license: "MIT"
use_eslint: true
use_prettier: true
```

### Step 2: Generate Project

```bash
# Preview with dry-run
scaffold generate \
  --template node-basic \
  --out node-express-api \
  --vars api-vars.yaml \
  --dry-run

# Apply changes
scaffold generate \
  --template node-basic \
  --out node-express-api \
  --vars api-vars.yaml \
  --apply

# With script execution (for npm install)
scaffold generate \
  --template node-basic \
  --out node-express-api \
  --vars api-vars.yaml \
  --apply \
  --allow-scripts

# Auto-commit git repo
scaffold generate \
  --template node-basic \
  --out node-express-api \
  --vars api-vars.yaml \
  --apply \
  --allow-scripts \
  --commit
```

### Step 3: Setup and Run

```bash
cd node-express-api

# Install dependencies (if not using --allow-scripts)
npm install

# Run development server
npm run dev

# Run tests
npm test

# Build for production
npm run build

# Lint code
npm run lint

# Format code
npm run format
```

## 🔧 Advanced Configuration

### Multiple Environments

```yaml
# dev-vars.yaml
project_name: "node-api-dev"
description: "Development API server"
use_eslint: true
use_prettier: true

# prod-vars.yaml
project_name: "node-api-prod"
description: "Production API server"
use_eslint: true
use_prettier: false
```

```bash
# Environment-based generation
if [ "$NODE_ENV" = "production" ]; then
  scaffold generate --template node-basic --out . --vars prod-vars.yaml --apply
else
  scaffold generate --template node-basic --out . --vars dev-vars.yaml --apply
fi
```

### Database Integration

```yaml
# database-vars.yaml
project_name: "node-db-api"
description: "Node.js API with PostgreSQL integration"
use_eslint: true
use_prettier: true
```

```bash
# Generate project
scaffold generate \
  --template node-basic \
  --out node-db-api \
  --vars database-vars.yaml \
  --apply \
  --allow-scripts

# Add database dependencies
cd node-db-api
npm install pg @types/pg dotenv

# Update package.json
cat >> package.json << 'EOF'

"scripts": {
  "db:migrate": "node scripts/migrate.js",
  "db:seed": "node scripts/seed.js"
},
EOF
```

### Custom Dependencies

```yaml
# custom-vars.yaml
project_name: "node-custom-app"
description: "Custom Node.js application with additional dependencies"
author: "Your Name <you@example.com>"
license: "MIT"
use_eslint: true
use_prettier: true
```

```bash
# Generate and extend
scaffold generate \
  --template node-basic \
  --out node-custom-app \
  --vars custom-vars.yaml \
  --apply \
  --allow-scripts

# Add extra dependencies
cd node-custom-app
npm install express cors helmet morgan

# Update tsconfig.json for paths
cat >> tsconfig.json << 'EOF'

"compilerOptions": {
  "baseUrl": "./src",
  "paths": {
    "@/*": ["*"],
    "@/config/*": ["config/*"]
  }
},
EOF
```

## 📱 Generated Project Structure

```
node-express-api/
├── .git/
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
├── .eslintrc.json
├── .prettierrc
├── node_modules/
├── src/
│   ├── index.test.ts
│   └── index.ts
├── package.json
├── package-lock.json
├── README.md
├── tsconfig.json
└── dist/
```

## 🚀 Development Workflow

### Initial Setup

```bash
cd node-express-api

# Dependencies installed by --allow-scripts
npm install

# Start development server
npm run dev
# Output: Server running on http://localhost:3000
```

### Code Structure

```typescript
// src/index.ts - Application entry point
import express, { Request, Response } from 'express';

const app = express();
const PORT = process.env.PORT || 3000;

app.get('/', (req: Request, res: Response) => {
  res.json({ message: 'Hello from {{project_name}}!' });
});

app.listen(PORT, () => {
  console.log(`🚀 Server running on port ${PORT}`);
});
```

### Testing

```typescript
// src/index.test.ts - Test file
import { greet } from './index';

describe('greet', () => {
  it('should return a greeting message', () => {
    expect(greet('World')).toBe('Hello, World!');
  });

  it('should include project name in greeting', () => {
    expect(greet('{{project_name}}')).toContain('{{project_name}}');
  });
});
```

## 🔄 CI/CD Pipeline

### GitHub Actions Workflow

Generated `.github/workflows/ci.yml`:

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
    strategy:
      matrix:
        node-version: [18.x, 20.x]
    
    steps:
      - uses: actions/checkout@v4
      - name: Use Node.js ${{ matrix.node-version }}
        uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Lint
        run: npm run lint
      
      - name: Build
        run: npm run build
      
      - name: Test
        run: npm test
```

### Local CI Testing

```bash
# Install act for local GitHub Actions
go install -u github.com/nektos/act@latest

# Run CI locally
act -j ubuntu-latest

# Run specific job
act -j ubuntu-latest -P build.node-version=18.x
```

## 🔍 Advanced Development

### Environment Configuration

```typescript
// src/config/index.ts
import dotenv from 'dotenv';

dotenv.config();

export const config = {
  port: parseInt(process.env.PORT || '3000', 10),
  nodeEnv: process.env.NODE_ENV || 'development',
  database: {
    host: process.env.DB_HOST || 'localhost',
    port: parseInt(process.env.DB_PORT || '5432', 10),
    name: process.env.DB_NAME || 'myapp',
  },
};
```

### Middleware Setup

```typescript
// src/middleware/index.ts
import express, { Request, Response, NextFunction } from 'express';
import cors from 'cors';
import helmet from 'helmet';
import morgan from 'morgan';

export const setupMiddleware = (app: express.Application): void => {
  // Security
  app.use(helmet());
  
  // CORS
  app.use(cors({
    origin: process.env.ALLOWED_ORIGINS?.split(',') || ['http://localhost:3000'],
    credentials: true,
  }));
  
  // Logging
  app.use(morgan('combined'));
  
  // Body parsing
  app.use(express.json());
  app.use(express.urlencoded({ extended: true }));
};
```

### API Routes

```typescript
// src/routes/index.ts
import { Router } from 'express';
import { healthCheck } from './health';
import { apiRoutes } from './api';

const router = Router();

router.get('/health', healthCheck);
router.use('/api', apiRoutes);

export { router };
```

### Error Handling

```typescript
// src/middleware/errorHandler.ts
import { Request, Response, NextFunction } from 'express';

export interface AppError extends Error {
  statusCode?: number;
  isOperational?: boolean;
}

export const errorHandler = (
  err: AppError,
  req: Request,
  res: Response,
  next: NextFunction,
): void => {
  const statusCode = err.statusCode || 500;
  const message = err.message || 'Internal Server Error';

  if (process.env.NODE_ENV === 'development') {
    console.error(err);
  }

  res.status(statusCode).json({
    error: message,
    ...(process.env.NODE_ENV === 'development' && { stack: err.stack }),
  });
};
```

## 🔍 Testing Strategy

### Unit Tests

```bash
# Run all tests
npm test

# Run in watch mode
npm run test:watch

# Generate coverage
npm run test:coverage

# View coverage report
open coverage/lcov-report/index.html
```

### Integration Tests

```bash
# Add integration test dependencies
npm install --save-dev supertest @types/supertest

# Create integration tests
mkdir -p tests/integration

# tests/integration/api.test.ts
import request from 'supertest';
import app from '../../src/index';

describe('API Integration Tests', () => {
  it('should return health check', async () => {
    const response = await request(app)
      .get('/health')
      .expect(200);
    
    expect(response.body).toHaveProperty('status');
  });
});
```

### End-to-End Tests

```bash
# Add E2E test dependencies
npm install --save-dev playwright @playwright/test

# Configure Playwright
npx playwright install

# Run E2E tests
npm run test:e2e
```

## 🚀 Production Deployment

### Build Process

```bash
# Production build
npm run build

# Check dist directory
ls -la dist/

# Test production build
NODE_ENV=production npm start
```

### Docker Deployment

```dockerfile
# Dockerfile (create after generation)
FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

EXPOSE 3000

USER node

CMD ["npm", "start"]
```

```bash
# Build and run Docker
docker build -t node-express-api .
docker run -p 3000:3000 node-express-api
```

### Cloud Platform Examples

#### Railway
```bash
# Deploy to Railway
npm install -g @railway/cli
railway login
railway up
```

#### Vercel
```bash
# Deploy to Vercel
npm install -g vercel
vercel --prod
```

#### AWS Lambda

```bash
# Add serverless dependencies
npm install --save-dev serverless serverless-http

# serverless.yml (create)
service: node-express-api

provider:
  name: aws
  runtime: nodejs18.x

functions:
  api:
    handler: dist/lambda.handler
    events:
      - http:
          path: /{proxy+}
          method: ANY
```

## 🔍 Troubleshooting

### TypeScript Errors

```bash
# Check TypeScript version
npx tsc --version

# Rebuild after changes
npm run build

# Check tsconfig.json
npx tsc --noEmit --pretty
```

### ESLint Issues

```bash
# Run with detailed output
npm run lint -- --format verbose

# Fix automatically
npm run lint:fix

# Check specific rules
npx eslint src/index.ts --rule 'no-unused-vars'
```

### Dependency Conflicts

```bash
# Check for conflicts
npm ls

# Resolve version conflicts
npm install package@^1.2.0

# Clean install
rm -rf node_modules package-lock.json
npm install
```

### Node Version Issues

```bash
# Check Node version
node --version

# Use nvm for version management
nvm use 18
nvm install 20

# Set default version
nvm alias default 18
```

## 📚 Next Steps

1. **Database Integration**: Add MongoDB or PostgreSQL
2. **Authentication**: Add JWT or OAuth2
3. **API Documentation**: Add Swagger/OpenAPI
4. **Rate Limiting**: Add request throttling
5. **Monitoring**: Add health checks and metrics
6. **Testing**: Add comprehensive test coverage
7. **Security**: Add input validation and sanitization
8. **Performance**: Add caching and optimization

## 🔗 Related Examples

- [Rust Project](rust-project.md) - Similar setup for Rust
- [Custom Template](custom-template.md) - Create your own template
- [CI Integration](../getting-started.md#adding-ci-to-existing-project) - Add CI to existing projects
- [Testing Guide](../getting-started.md#testing-generated-projects) - Test your generated project

## 🆘 Common Issues

### **Issue**: npm install fails during generation
```
Error: command blocked by network policy: npm install
```

**Solution**:
```bash
# Allow network access for node templates
scaffold generate --template node-basic --out my-app --apply --allow-scripts
```

### **Issue**: TypeScript compilation errors
```
error TS2307: Cannot find module 'express'
```

**Solution**:
```bash
# Install types
npm install --save-dev @types/express @types/node

# Update tsconfig.json
echo '"moduleResolution": "node"' >> tsconfig.json
```

### **Issue**: Port already in use
```
Error: listen EADDRINUSE :::3000
```

**Solution**:
```bash
# Kill process on port
lsof -ti:3000 | xargs kill -9

# Use different port
PORT=3001 npm run dev
```

### **Issue**: Tests fail in CI
```
FAIL test
  ● greet
    Expected: "Hello, World!"
    Received: undefined
```

**Solution**:
```bash
# Check test imports
cat src/index.test.ts

# Verify module exports
cat src/index.ts

# Run tests with coverage
npm run test:coverage
```
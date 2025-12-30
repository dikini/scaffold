# Example Workflow: Mobile Expense Tracking App

This example demonstrates how the Stage 2 skills work together to take a product idea from concept to implementation-ready specifications.

## 🚀 Step 1: Create PRD
```bash
skill write-prd
```
**Input**: "I want to build a mobile expense tracking app for freelancers"
**Output**: Comprehensive PRD with:
- Problem: Freelancers struggle with expense tracking and tax preparation
- Target Users: Independent contractors, gig economy workers
- Features: Receipt scanning, categorization, tax reporting
- Success Metrics: User retention, accurate expense categorization

## 🔍 Step 2: Analyze PRD
```bash
skill analyze-prd
```
**Input**: The PRD from Step 1
**Output**: Implementation analysis with:
- Risk Assessment: OCR accuracy, data privacy concerns
- Resource Requirements: 2-3 developers, 6 month timeline
- Implementation Phases: MVP → Advanced features → Integrations
- Technical Recommendations: Cloud-based processing, mobile-first design

## 📋 Step 3: Feature Mapping
```bash
skill feature-mapping
```
**Input**: PRD and analysis from previous steps
**Output**: Agile work breakdown:
- **Epic**: User Management
  - Feature: Registration & Authentication
    - User Stories: Sign up with email, Social login, Password reset
- **Epic**: Expense Tracking
  - Feature: Receipt Capture
    - User Stories: Scan receipt, Manual entry, Category selection
- **Story Points**: 8-13 points per story
- **Sprint Recommendations**: 3-week sprints with 20-30 points capacity

## 🏗️ Step 4: Technical Design
```bash
skill tech-design-doc
```
**Input**: Requirements and feature breakdown
**Output**: Technical specifications:
- **Architecture**: Mobile app + REST API + Cloud storage
- **Tech Stack**: React Native, Node.js, PostgreSQL, AWS S3
- **API Design**: 15 endpoints for users, expenses, categories
- **Database**: Schema with 8 tables, normalized design
- **Security**: JWT authentication, encrypted data storage

## 📅 Step 5: Project Roadmap
```bash
skill project-roadmap
```
**Input**: Technical design and feature breakdown
**Output**: Visual timeline:
- **Month 1-2**: User authentication, basic expense entry
- **Month 3-4**: Receipt scanning, OCR integration
- **Month 5-6**: Tax reporting, analytics dashboard
- **Release Schedule**: Beta release at Month 4, Public launch at Month 6

## 🔌 Step 6: API Contracts (Optional)
```bash
skill api-contract
```
**Input**: Technical design with API specifications
**Output**: Complete API documentation:
- OpenAPI 3.0 specification
- 15 endpoints with request/response examples
- Authentication flow documentation
- Error handling and rate limiting guidelines

## 📊 Final Deliverables

After completing this workflow, you have:

1. **PRD Document** - Business requirements and success criteria
2. **Implementation Plan** - Phased approach with risk mitigation
3. **Agile Backlog** - Sprint-ready user stories with acceptance criteria
4. **Technical Specs** - Architecture, database design, and API specifications
5. **Project Timeline** - Visual roadmap with milestones and releases
6. **API Documentation** - Complete contract for frontend/backend integration

This comprehensive package is ready for:
- **Stakeholder Approval** - Clear business case and implementation plan
- **Development Team Onboarding** - Complete technical specifications
- **Project Management** - Sprint planning and milestone tracking
- **Quality Assurance** - Acceptance criteria and testing guidelines

## 🎯 Integration with Scaffold

The final specifications can directly inform scaffold template generation:

```bash
scaffold generate --template node-basic \
  --out expense-tracker \
  --vars project_vars.yaml \
  --apply
```

Where `project_vars.yaml` contains values derived from the skill outputs:
```yaml
project_name: expense-tracker-pro
description: Mobile expense tracking for freelancers
features: ["receipt-scanning", "tax-reporting", "analytics"]
tech_stack: ["react-native", "nodejs", "postgresql"]
```

This demonstrates how Stage 2 skills transform scaffold from just "how to build" to the complete "what to build, why to build it, and how to plan the build."
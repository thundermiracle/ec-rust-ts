# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a modern e-commerce application built with Rust backend and Next.js frontend, managed as a monorepo using Nx.

## Architecture

### Backend (Rust)
- **Framework**: Axum web framework with async/await
- **Database**: SQLite with SQLx
- **Architecture**: Clean Architecture with CQRS pattern
- **Runtime**: Tokio async runtime

### Frontend (Next.js)
- **Framework**: Next.js 15 with App Router
- **Runtime**: React 19
- **Styling**: Tailwind CSS with Radix UI components
- **State Management**: Redux Toolkit with RTK Query

## Common Commands

### Development
```bash
# Start both frontend and backend in development mode
pnpm dev

# Start individual services
pnpm frontend:dev    # Next.js dev server
pnpm backend:dev     # Rust backend with hot reload
```

### Building
```bash
# Build all applications
pnpm build

# Build individual applications
pnpm frontend:build
pnpm backend:build
```

### Testing
```bash
# Run all tests
pnpm test

# Run specific tests
pnpm frontend:test
pnpm backend:test

# Run single test file (backend)
cd app/backend && cargo test integration_product_repository_test
```

### Linting and Formatting
```bash
# Lint all code
pnpm lint

# Individual linting
pnpm frontend:lint
pnpm backend:lint

# Format Rust code
pnpm backend:format
cd app/backend && cargo fmt
```

### Backend Development
```bash
# Watch mode for development
pnpm backend:watch

# Type checking
pnpm backend:check

# Build and run
cd app/backend && cargo build --release
cd app/backend && cargo run
```

## Backend Architecture (Clean Architecture)

### Layer Structure
```
src/
├── domain/              # Business entities and rules
├── application/         # Use cases and business logic
├── infrastructure/      # External systems and database
└── presentation/        # HTTP controllers and routes
```

### Key Patterns
- **CQRS**: Commands (write operations) and Queries (read operations) are separated
- **Repository Pattern**: Data access abstraction
- **Dependency Injection**: Container-based DI system
- **Error Handling**: Layered error types with proper propagation

### Adding New Endpoints
The project follows strict naming conventions and structure patterns:

**Command Endpoints (POST/PUT/DELETE)**:
- Follow the create-command-instructions.mdc in .cursor/rules/
- Use pattern: `{action}_{entity}_command.rs` → `{Action}{Entity}Command`
- Example: `calculate_cart_command.rs` → `CalculateCartCommand`

**Query Endpoints (GET)**:
- Follow the create-query-instructions.mdc in .cursor/rules/
- Use pattern: `get_{entity}_list_handler.rs` → `Get{Entity}ListHandler`
- Example: `get_product_list_handler.rs` → `GetProductListHandler`

### Database
- SQLite database located at `app/backend/data/db.sqlite`
- Migrations in `app/backend/migrations/`
- Schema documentation in `app/backend/docs/database-schema.md`

## Frontend Architecture

### State Management
- Redux Toolkit store in `src/store/`
- RTK Query for API calls with code generation
- Auto-generated API client from backend OpenAPI spec

### Components
- Reusable UI components in `src/components/ui/`
- Feature-specific components organized by domain
- Tailwind CSS with design system approach

### API Integration
- Generated API client in `src/store/generatedApi/`
- Run `pnpm frontend:build` to regenerate API types

## Key Files and Locations

### Backend
- Main entry: `app/backend/src/main.rs`
- Dependencies: `app/backend/Cargo.toml`
- Container/DI: `app/backend/src/infrastructure/di/container.rs`
- Routes: `app/backend/src/presentation/routes.rs`
- OpenAPI: `app/backend/src/presentation/swagger/openapi.rs`

### Frontend
- Main entry: `app/frontend/src/app/page.tsx`
- Dependencies: `app/frontend/package.json`
- Store: `app/frontend/src/store/store.ts`
- API config: `app/frontend/rtk-query-codegen.config.ts`

## Development Guidelines

### Backend Development
- Follow Clean Architecture principles strictly
- Use async/await for all I/O operations
- Implement proper error handling with custom error types
- Add comprehensive logging with `println!` for debugging
- Write unit tests for handlers and presenters

### Frontend Development
- Use TypeScript strictly
- Follow React 19 patterns and hooks
- Implement proper loading and error states
- Use RTK Query for all API calls
- Follow Tailwind CSS conventions

### Code Quality
- Run `cargo fmt` before committing Rust code
- Run `cargo clippy` to check for warnings
- Use `bacon` for continuous compilation checking
- Follow the architecture patterns established in .cursor/rules/

## Testing

### Backend Tests
- Unit tests alongside source files
- Integration tests in `app/backend/tests/`
- Use `cargo test` for all tests
- Mock repositories for testing use cases

### Frontend Tests
- Unit tests with Vitest
- Component tests with React Testing Library
- Test setup in `app/frontend/src/test/setup.ts`

## Deployment

Built as a monorepo with Nx orchestration:
- Backend compiles to single binary
- Frontend builds to static files
- Both can be deployed independently

## Monitoring and Development Tools

- OpenAPI/Swagger UI available at `http://localhost:4000/swagger-ui/`
- Backend runs on port 4000
- Frontend runs on port 3000
- Database can be inspected with SQLite tools
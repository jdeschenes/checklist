# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Prerequisites

Install these tools for development:
- docker
- nvm/npm
- cargo
- sqlx-cli (installed via `cargo install sqlx-cli --no-default-features --features rustls,postgres`)

## Development Commands

### Initial Setup
```bash
make initial-setup  # Or ./scripts/setup.sh
```

### Database Management
```bash
make start-db    # Start PostgreSQL in Docker
make stop-db     # Stop PostgreSQL container
```

### Development Server
```bash
make run         # Starts frontend dev server (background) + backend server
# Or manually: cd frontend && npm run dev & cd ../backend && cargo run
```

### Build Commands
```bash
make build       # Build both frontend and backend for production
```

### Testing
```bash
make test        # Run backend Rust tests
make golden-test # Run tests with golden file overwrite (GOLDEN_OVERWRITE=true)
```

To get useful logging information about a particular test you can set the
environment variable TEST_ENV=true to get logs printed to stdout

### Frontend-Specific Commands
```bash
cd frontend
npm run dev      # Development server
npm run build    # Production build
npm run check    # TypeScript type checking
npm run lint     # ESLint + Prettier check
npm run format   # Auto-format with Prettier
```

### Git Commit Guidelines

This project follows [Conventional Commits](https://conventionalcommits.org/) specification:

```bash
# Format: <type>(<scope>): <description>
# 
# <body>
# 
# <footer>
```

**Common Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, no logic changes)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```bash
feat: add todo item completion with debounced UX
fix: resolve timezone issues in date display  
docs: update API documentation
refactor: simplify date formatting logic
```

## Architecture Overview

This is a full-stack todo/checklist application with:

### Backend (Rust/Axum)
- **Framework**: Axum web framework with tokio async runtime
- **Database**: PostgreSQL with SQLx for database operations
- **Structure**: Domain-driven design with separate layers:
  - `domain/`: Business entities (todo.rs, todo_item.rs)  
  - `repos/`: Repository layer for data access
  - `routes/`: HTTP route handlers
  - `configuration.rs`: Config management with environment-specific YAML files
  - `telemetry.rs`: Structured logging with tracing
- **Testing**: Integration tests with golden file testing in `tests/api/`
- **Configuration**: Environment-specific YAML files in `configuration/`

### Frontend (React/TypeScript)
- **Framework**: React 18 with Vite build tool
- **Routing**: TanStack Router with file-based routing in `src/routes/`
- **State Management**: TanStack Query for server state
- **Styling**: Tailwind CSS v4 with Radix UI components
- **UI Components**: Custom components in `src/components/ui/` using class-variance-authority
- **API Layer**: Centralized in `src/api/` with query options and custom hooks

### Key Patterns
- **Route Structure**: `/todo/:todoId` for todo details, `/todo/:todoId/new` for creating items
- **API Design**: RESTful endpoints with JSON responses
- **Error Handling**: Custom error types in backend, React Query error boundaries in frontend
- **Database**: Migration-based schema management with SQLx migrations
- **Testing**: Golden file testing for API responses ensures consistent output

## Database Schema

The application manages todos and todo items with a hierarchical relationship where each todo can contain multiple todo items.

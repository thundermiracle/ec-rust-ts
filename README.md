# EC-Rust-TS

A modern e-commerce application built with Rust backend and Next.js frontend, managed as a monorepo using Nx.

## 🏗️ Architecture

This project follows a clean architecture pattern with clear separation of concerns:

- **Backend**: Rust with Axum web framework, SQLite database, and async/await patterns
- **Frontend**: Next.js 15 with React 19, TypeScript, Tailwind CSS, and Radix UI components
- **Monorepo**: Managed with Nx for efficient development workflows

## 📁 Project Structure

```
ec-rust-ts/
├── app/
│   ├── backend/        # Rust backend (Axum + SQLite)
│   └── frontend/       # Next.js frontend (React + TypeScript)
├── package.json        # Root package.json with workspace scripts
└── nx.json            # Nx configuration
```

## 🚀 Getting Started

### Prerequisites

- Node.js >= 18.0.0
- Rust (latest stable version)
- pnpm (recommended package manager)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd ec-rust-ts
```

2. Install dependencies:
```bash
pnpm install
```

### Development

#### Start both frontend and backend:
```bash
pnpm dev
```

#### Start individual services:

**Frontend only:**
```bash
pnpm frontend:dev
```

**Backend only:**
```bash
pnpm backend:dev
```

### Building

#### Build all applications:
```bash
pnpm build
```

#### Build individual applications:
```bash
pnpm frontend:build
pnpm backend:build
```

## 🔧 Available Scripts

### Root Level Scripts
- `pnpm dev` - Start all applications in development mode
- `pnpm build` - Build all applications
- `pnpm test` - Run all tests
- `pnpm lint` - Lint all applications

### Frontend Scripts
- `pnpm frontend:dev` - Start frontend development server
- `pnpm frontend:build` - Build frontend for production
- `pnpm frontend:start` - Start frontend production server
- `pnpm frontend:lint` - Lint frontend code

### Backend Scripts
- `pnpm backend:dev` - Start backend development server
- `pnpm backend:build` - Build backend for production
- `pnpm backend:test` - Run backend tests
- `pnpm backend:watch` - Watch mode for backend development

## 🛠️ Technology Stack

### Backend
- **Framework**: Axum (Rust web framework)
- **Database**: SQLite with SQLx
- **Runtime**: Tokio (async runtime)
- **Serialization**: Serde

### Frontend
- **Framework**: Next.js 15 with App Router
- **Runtime**: React 19
- **Styling**: Tailwind CSS
- **UI Components**: Radix UI
- **Icons**: Lucide React

### Development Tools
- **Monorepo**: Nx
- **Package Manager**: pnpm
- **Linting**: ESLint
- **Type Checking**: TypeScript

## 📖 Documentation

For detailed architecture information, see the [backend documentation](./app/backend/docs/ARCHITECTURE.md).

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

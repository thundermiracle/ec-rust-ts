# E-commerce Backend - Clean Architecture

A Rust-based e-commerce backend following Clean Architecture principles with normalized database schema supporting rich product data.

## 📚 Documentation

- **[Database Schema](docs/database-schema.md)** - Complete ER diagram and table descriptions
- **[Clean Architecture Guidelines](docs/ARCHITECTURE.md)** - Project architecture and coding standards

## 🚀 Quick Start

### Install Dependencies

```shell
cargo install --locked bacon
```

### Database Setup

```shell
# Create SQLite database
mkdir -p data
touch data/db.sqlite

# Run migrations (creates all tables)
cargo run -- migration

# Seed sample data (mockData from frontend)
cargo run -- seed
```

### Development

```shell
# Terminal1: Watch web server with hot reload
bacon dev

# Terminal2: Watch HTTP tests
bacon http-test
```

### Available Commands

```shell
# Database operations
cargo run -- migration    # Create/update database schema
cargo run -- seed        # Insert sample data
cargo run -- reset       # Clear all data and reseed

# Server
cargo run                 # Start production server
cargo run -- dev         # Start development server
```

# Prism

A high-performance system metrics collection framework written in Rust.

## Overview

Prism collects and processes system metrics including CPU, memory, disk, and network statistics. Built with Rust for performance and reliability, it provides modular metric collection with flexible output formats.

## Features

- **Modular Architecture**: CPU, memory, disk, network metric modules
- **High Performance**: Zero-cost abstractions and minimal overhead
- **Flexible Output**: JSON, CSV, and custom format support
- **Comprehensive Testing**: Integration tests with synthetic data validation

## Quick Start

```bash
# Clone and build
git clone https://github.com/DeepShield-AI/prism.git
cd prism
cargo build --release

# Run application
cargo xtask run

# Run tests
cargo test
cargo run --bin procfs_integration_tests
```

## Documentation

- **[User Guides](docs/README.md)** - Configuration, testing, and usage
- **[Development](docs/src/development.md)** - Architecture and contribution guide  
- **[Testing Guide](docs/src/testing.md)** - Quick testing reference
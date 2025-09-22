# Getting Started

This guide will help you get up and running with Prism quickly, from initial setup to running your first metric collection.

## Prerequisites

Before installing Prism, ensure you have the following:

### System Requirements
- **Operating System**: Linux (Ubuntu 24.04)
- **Architecture**: x86_64 or ARM64
- **Memory**: Minimum 512MB RAM (1GB+ recommended)
- **Disk Space**: 100MB for installation, additional space for logs and data

### Development Requirements
- **Rust**: Version 1.90.0 or later
- **Git**: For cloning the repository
- **Build Tools**: Standard development tools (gcc, make, etc.)

### Runtime Requirements
- **procfs**: Access to `/proc` filesystem (standard on Linux)
- **Permissions**: Read access to system files in `/proc`

## Setup development environment
You can setup development environment with [`Development Environment`](development.md) guide.

## Installation

### From Source

1. **Clone the Repository**
   ```bash
   git clone https://github.com/your-org/prism.git
   cd prism
   ```

2. **Build the Project**
   ```bash
   # Debug build (faster compilation)
   cargo xtask build
   
   # Release build (optimized performance)
   cargo xtask build --profile release
   ```

3. **Run Tests**
   ```bash
   # Run all tests to verify installation
   cargo test
   
   # Run integration tests
   cargo run --bin procfs_integration_tests -- --count 3 --verbose
   # The test results are saved in the output folder
   ```

## Next Steps

Now that you have Prism running:

<!-- 1. **Explore Configuration**: Learn about advanced configuration options -->
1. **Review Architecture**: Understand how Prism works internally
2. **Run Tests**: Explore the comprehensive testing framework
3. **Customize Output**: Configure output formats and destinations
4. **Monitor Performance**: Use Prism to monitor your systems

For detailed information about Prism's architecture and components, see the [Architecture](./architecture.md) guide.

For comprehensive testing information, see the [Testing](./testing.md) section.

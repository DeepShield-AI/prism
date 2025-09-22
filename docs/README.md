# Prism Documentation

## User Guides
<!-- - **[Configuration](src/configuration.md)** - Configuration file format and options -->
- **[Testing](src/testing.md)** - Testing framework and validation

## Developer Guides  
- **[Architecture](src/architecture.md)** - System architecture and components
- **[Development](src/development.md)** - Development setup and guidelines

## Quick References
- **[Integration Tests](../tests/procfs-integration-tests/README.md)** - Test implementation details

## How to use this documentation
To view this documentation in browser, you need `cargo` installed on your system.
Then install `mdbook`:
```bash
cargo install mdbook
```
To build and serve the documentation, run
```bash
mdbook serve --open
```
Then open your browser and go to `http://localhost:3000`
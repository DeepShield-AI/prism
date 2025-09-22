# Development Guide

## Project Structure

```
prism/
├── crates/           # Core crates and modules
│   ├── prism/        # Main application
│   ├── prism-core/   # Core framework
│   ├── prism-cpu/    # CPU metrics
│   ├── prism-memory/ # Memory metrics
│   ├── prism-disk/   # Disk metrics
│   ├── prism-network/# Network metrics
│   ├── prism-event/  # Event system
│   ├── codec/        # Data encoding
│   └── ...
├── tests/            # Integration tests
├── docs/             # Documentation
├── config/           # Configuration files
└── xtask/            # Build automation
```

## Prerequisites
First, you will need some base dependencies installed on your system.
```bash
sudo apt-get update && sudo apt-get install -y --no-install-suggests --no-install-recommends \
  build-essential clang curl ca-certificates git make libelf-dev
```

Before getting started you will need the Rust stable and nightly toolchains installed on your system. This is easily achieved with `rustup`:
```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable
echo "export PATH=$PATH:$HOME/.cargo/bin" >> ~/.bashrc
source ~/.bashrc

# Verify installation
rustc --version
cargo --version

# Install toolchains
rustup toolchain install 1.90.0
rustup default 1.90.0

rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# Add target
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
```
Once you have the Rust toolchains installed, you must also install `bpf-linker`. The linker depends on LLVM, and it can be built against the version shipped with the rust toolchain with:
```bash
cargo install bpf-linker
```
If you want to read documents locally, you can install `mdbook` with:
```bash
cargo install mdbook
```

### Development Workflow
```bash
# Format code
cargo +nightly fmt

# Run linter
cargo xtask clippy

# Run all tests
cargo test

# Run integration tests
cargo run --bin procfs_integration_tests

# Check for issues
cargo check

# Run prism
RUST_LOG=info cargo xtask run --profile release
```

## Adding New Metrics

### 1. Create New Crate
```bash
# Create new metric module
mkdir crates/prism-newmetric
cd crates/prism-newmetric

# Initialize Cargo.toml
cargo init --lib
```

### 2. Implement Metric Collection
```rust
// src/lib.rs
use prism_metric_common::procfs_root;
use prism_metric_utils::read_to_string;

pub async fn collect_metric() -> Result<MetricData, MetricError> {
    let content = read_to_string(procfs_root().join("newmetric")).await?;
    parse_metric(&content)
}

fn parse_metric(content: &str) -> Result<MetricData, MetricError> {
    // Implementation
}
```

### 3. Add Integration Tests
```rust
// tests/prism-fixtures/src/generators/newmetric.rs
use fake::{Dummy, Fake, Faker};

#[derive(Debug, Dummy)]
pub struct FakeNewMetric {
    pub field1: u64,
    pub field2: String,
}

impl FakeNewMetric {
    pub fn generate() -> Self {
        Faker.fake()
    }
}

impl ToString for FakeNewMetric {
    fn to_string(&self) -> String {
        format!("field1: {}\nfield2: {}\n", self.field1, self.field2)
    }
}
```

### 4. Add Validation
```rust
// tests/procfs-integration-tests/src/validators.rs
impl Validator {
    pub fn validate_newmetric(&self, fake: FakeNewMetric, real: NewMetric) -> Result<()> {
        assert_eq!(fake.field1, real.field1, "Field1 mismatch");
        assert_eq!(fake.field2, real.field2, "Field2 mismatch");
        Ok(())
    }
}
```

## Debugging

### Environment Variables
```bash
# Enable debug logging
export RUST_LOG=debug

# Custom procfs root for testing
export PROCFS_ROOT=/tmp/fake-proc

# Backtrace on panic
export RUST_BACKTRACE=1
```

## Contributing

### Pull Request Process
1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Run full test suite
5. Submit pull request with description

### Code Review Checklist
- [ ] Code follows style guidelines
- [ ] Tests cover new functionality
- [ ] Documentation is updated
- [ ] Performance impact is considered
- [ ] Error handling is appropriate

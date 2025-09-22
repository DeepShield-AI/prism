# Unit Tests

Unit tests form the foundation of Prism's testing strategy, providing focused validation of individual components, functions, and modules. These tests ensure that each piece of functionality works correctly in isolation before being integrated into the larger system.

## Testing Strategy

### Component Isolation

Unit tests focus on testing individual components in isolation:
- **Pure Functions**: Test mathematical calculations and data transformations
- **Data Structures**: Validate custom data types and their operations
- **Parsing Logic**: Verify correct interpretation of procfs file formats
- **Error Handling**: Ensure proper error detection and recovery

### Test Coverage Goals

- **Functionality Coverage**: Every public function and method
- **Branch Coverage**: All conditional logic paths
- **Error Coverage**: All error conditions and edge cases
- **Performance Coverage**: Critical path performance validation

## Test Organization

### Module Structure

Unit tests are organized alongside the code they test:

```
crates/
├── prism-cpu/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── stat.rs
│   │   └── ...
│   └── tests/
│       ├── integration.rs
│       └── unit/
│           ├── stat_tests.rs
│           └── ...
├── prism-memory/
│   ├── src/
│   └── tests/
└── ...
```

## Running Unit Tests

### Basic Test Execution

```bash
# Run all unit tests
cargo test

# Run tests for specific crate
cargo test -p prism-cpu

# Run specific test
cargo test test_cpu_stat_parsing

# Run tests with output
cargo test -- --nocapture
```

### Advanced Test Options

```bash
# Run tests in release mode (for performance testing)
cargo test --release

# Run tests with specific number of threads
cargo test -- --test-threads=1

# Run ignored tests
cargo test -- --ignored

# Show test execution time
cargo test -- --report-time
```

### Test Coverage

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Coverage for specific crate
cargo tarpaulin -p prism-cpu --out Html
```

## Best Practices

### Test Organization
- **One Test Per Behavior**: Each test should verify one specific behavior
- **Descriptive Names**: Test names should clearly describe what is being tested
- **Arrange-Act-Assert**: Structure tests with clear setup, execution, and verification phases

### Test Maintenance
- **Keep Tests Simple**: Tests should be easy to understand and maintain
- **Avoid Test Dependencies**: Tests should not depend on each other
- **Regular Review**: Periodically review and update tests as code evolves

### Performance Considerations
- **Fast Execution**: Unit tests should run quickly to encourage frequent execution
- **Minimal Setup**: Reduce test setup overhead where possible
- **Parallel Execution**: Design tests to run safely in parallel

For comprehensive end-to-end testing, see [Integration Tests](./integration-tests.md).

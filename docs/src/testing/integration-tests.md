# Integration Tests

Prism's integration testing framework is a sophisticated system designed to validate the complete metric collection pipeline from data generation to final output. The integration tests ensure that all components work together correctly and that the system produces accurate results under various conditions.

## Overview

The integration testing framework, located in `tests/procfs-integration-tests/`, provides comprehensive end-to-end validation of Prism's metric collection capabilities. It generates realistic procfs data, processes it through Prism's collection modules, and validates that the results match expected values with perfect accuracy.

## Architecture

### Test Framework Components

```
Integration Test Framework
├── Data Generation
│   ├── Random procfs file generation
│   └── Realistic system resource simulation
├── Metric Collection
│   ├── Prism module invocation
│   ├── Environment isolation
│   └── Process separation
└── Validation
    ├── Field-by-field verification
    ├── Unit conversion validation
    └── Performance measurement
```

### Key Features

- **Random Data Generation**: Creates realistic but controlled test data
- **Process Isolation**: Each test runs in a separate process to avoid conflicts
- **Comprehensive Validation**: Verifies every collected metric field
- **Performance Monitoring**: Tracks collection performance and overhead
  
## Test Data Generation

### Supported Metrics

The framework generates test data for all major system metrics:

#### CPU Metrics (`/proc/stat`)
- Random CPU core count (1-16 cores)
- Context switch statistics
- Process and thread counts
- Boot time and system uptime

#### Memory Metrics (`/proc/meminfo`)
- Total memory size
- Realistic memory usage patterns
- Cache and buffer allocations
- Swap space configuration
- Active/inactive memory distribution

#### Virtual Memory Statistics (`/proc/vmstat`)
- Page allocation and deallocation statistics
- Memory pressure indicators
- NUMA topology statistics
- I/O and swap activity metrics
- Slab cache utilization

#### Disk Metrics (`/proc/diskstats`)
- Multiple device types (SATA, NVMe, loop devices)
- Read/write operation statistics
- I/O timing and queue depth metrics
- Sector-level transfer statistics

#### Network Metrics (`/proc/net/dev`)
- Traffic statistics (bytes, packets)
- Error and drop counters
- Realistic usage patterns

### Data Generation Process

1. **Resource Allocation**: Generate realistic system resource configurations
2. **File Creation**: Create complete procfs file structures
3. **Validation Data**: Store expected values for later verification
4. **Environment Setup**: Configure test environment with generated data

## Test Execution

### Command Line Interface

The integration test framework provides a modern command-line interface:

```bash
# Basic usage
cargo run --bin procfs_integration_tests

# Multiple test runs
cargo run --bin procfs_integration_tests -- --count 5

# Verbose output with detailed validation
cargo run --bin procfs_integration_tests -- --count 3 --verbose

# Custom output directory
cargo run --bin procfs_integration_tests -- --output custom-results

# Show help and version information
cargo run --bin procfs_integration_tests --help
cargo run --bin procfs_integration_tests --version
```

### Test Process

Each integration test follows this workflow:

1. **Environment Preparation**
   - Create isolated test directory
   - Generate random procfs data
   - Set environment variables for procfs root

2. **Metric Collection**
   - Initialize Prism metric collection modules
   - Invoke collection functions (e.g., `prism_cpu::stat()`)
   - Capture all collected metrics

3. **Validation**
   - Compare collected values with generated expected values
   - Verify unit conversions and data transformations
   - Check field completeness and accuracy

4. **Result Recording**
   - Generate detailed validation reports
   - Record performance metrics

## Test Output and Reporting

### Directory Structure

Each test run creates a timestamped session directory:

```
output/20250920-183856/
├── test-001/
│   ├── procfs/              # Generated procfs files
│   │   ├── stat
│   │   ├── meminfo
│   │   ├── vmstat
│   │   ├── diskstats
│   │   └── net/dev
├── test-002/
└── test-003/
```

### Console Output

The test framework provides comprehensive console output:

```
Starting Prism ProcFS Random Integration Tests
==================================================
Running 3 tests
Test session directory: output/test-session-20250920-183856/

Starting Prism ProcFS Random Integration Tests
==================================================
Running 3 tests
Test session directory: output/20250922-142804/

Running test 1/3
  Test directory: output/20250922-142804/test-001/
  Running prism collectors and validating results
    Validating CPU metrics
      CPU field validation successful
    Validating Memory metrics
      Memory field validation successful
    Validating VmStat metrics
      VmStat field validation successful
    Validating Disk metrics
      Disk field validation successful
    Validating Network metrics
      
  Test #1 validation completed successfully
✅ Test #1 passed

...

Test session completed!
Results: 3 passed, 0 failed
All test results saved in: output/20250922-142804/
```

## Extending Integration Tests

### Adding New Metrics

To add support for new metric types:

1. **Generator Extension**: Add data generation logic in `generators.rs`
2. **Validator Implementation**: Create validation logic in `validators.rs`
3. **Test Integration**: Update main test loop to include new metrics
4. **Documentation**: Update test documentation and examples

### Custom Test Scenarios

The framework supports custom test scenarios:
- **Specific Resource Configurations**: Test particular system configurations
- **Edge Case Testing**: Focus on boundary conditions and error cases
- **Performance Testing**: Measure collection performance under load
- **Regression Testing**: Verify fixes for specific issues

### Configuration Options

Integration tests support various configuration options:
- **Test Count**: Number of test iterations to run
- **Output Directory**: Custom location for test results
- **Verbosity Level**: Control amount of output detail
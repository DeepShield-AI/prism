# Configuration Guide

## Overview

Prism uses TOML configuration files to customize behavior, output formats, and sampling intervals.

## Configuration File Format

### Basic Configuration

```toml
[metrics]
cpu_enabled = true
memory_enabled = true
disk_enabled = true
network_enabled = true

[output]
format = "json"
destination = "stdout"

[sampling]
interval_ms = 1000
```

### Detailed Options

#### Metrics Section
```toml
[metrics]
# Enable/disable metric collection modules
cpu_enabled = true
memory_enabled = true
disk_enabled = true
network_enabled = true
vmstat_enabled = true

# Module-specific settings
[metrics.cpu]
per_core = true
include_guest = false

[metrics.memory]
include_swap = true
include_cache_details = true

[metrics.disk]
include_partitions = true
sector_size = 512

[metrics.network]
include_errors = true
include_drops = true
```

#### Output Section
```toml
[output]
# Output format: "json", "csv", "custom"
format = "json"

# Destination: "stdout", "file", "network"
destination = "stdout"

# File output settings
[output.file]
path = "/var/log/prism/metrics.json"
rotate = true
max_size = "100MB"

# Network output settings
[output.network]
endpoint = "http://localhost:8080/metrics"
timeout_ms = 5000
```

#### Sampling Section
```toml
[sampling]
# Collection interval in milliseconds
interval_ms = 1000

# Buffer settings
buffer_size = 1000
flush_interval_ms = 5000

# Per-module intervals (optional)
[sampling.intervals]
cpu_ms = 1000
memory_ms = 2000
disk_ms = 5000
network_ms = 1000
```

## Environment Variables

Prism supports environment variable overrides:

```bash
# Override procfs root for testing
export PROCFS_ROOT=/custom/proc

# Override configuration file
export PRISM_CONFIG=/path/to/config.toml

# Override log level
export RUST_LOG=prism=debug
```

## Configuration Loading

1. Default configuration is loaded first
2. Configuration file is read (if specified)
3. Environment variables override file settings
4. Command-line arguments override all other settings

## Validation

Configuration is validated at startup:
- Required fields are checked
- Value ranges are validated
- Module dependencies are verified
- Output destinations are tested

## Examples

### High-Frequency Monitoring
```toml
[sampling]
interval_ms = 100

[metrics]
cpu_enabled = true
memory_enabled = false
disk_enabled = false
network_enabled = false
```

### Comprehensive Logging
```toml
[output]
format = "json"
destination = "file"

[output.file]
path = "/var/log/prism/all-metrics.json"
rotate = true

[metrics]
cpu_enabled = true
memory_enabled = true
disk_enabled = true
network_enabled = true
vmstat_enabled = true
```

### Development Testing
```toml
[sampling]
interval_ms = 5000

[output]
format = "json"
destination = "stdout"

[metrics]
cpu_enabled = true
memory_enabled = true
disk_enabled = false
network_enabled = false
```

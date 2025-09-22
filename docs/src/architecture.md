# Architecture

Prism follows a modular, event-driven architecture designed for high performance, extensibility, and maintainability. This document provides a comprehensive overview of the system's design, components, and data flow.

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Prism Architecture                       │
├─────────────────────────────────────────────────────────────────┤
│  Application Layer                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   CLI Interface │  │  Configuration  │  │    Logging      │  │
│  │                 │  │    Management   │  │   & Metrics     │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│  Core Framework                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │  Event System   │  │   Runtime       │  │   Scheduler     │  │
│  │                 │  │   Management    │  │                 │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│  Metric Collection Modules                                      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   CPU Module    │  │  Memory Module  │  │   Disk Module   │  │
│  │                 │  │                 │  │                 │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │ Network Module  │  │  Custom Modules │  │   Extensions    │  │
│  │                 │  │                 │  │                 │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│  Data Processing & Output                                       │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   Encoders      │  │   Senders       │  │   Storage       │  │
│  │  (JSON, CSV)    │  │  (File, Net)    │  │   Backends      │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│  System Interface                                               │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │     procfs      │  │     sysfs       │  │   Other APIs    │  │
│  │   (/proc/*)     │  │   (/sys/*)      │  │                 │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### Prism Core (`prism-core`)

The foundation of the Prism framework, providing:

- **Common Data Types**: Shared structures for metrics and system information
- **Error Handling**: Unified error types and handling strategies
- **Utility Functions**: Common functionality used across modules
- **Configuration Traits**: Interfaces for configuration management

### Event System (`prism-event`)

Manages the flow of data through the system using an event-driven architecture:

- **Event Types**: Defines various event types for different system activities
- **Event Bus**: Central event distribution mechanism
- **Subscribers**: Components that react to specific events
- **Publishers**: Components that emit events

### Runtime Management (`prism-runtime`)

Provides async runtime and execution management:

- **Task Scheduling**: Manages periodic metric collection tasks
- **Resource Management**: Controls system resource usage
- **Lifecycle Management**: Handles startup, shutdown, and restart scenarios
- **Concurrency Control**: Manages concurrent operations safely

### Data Transmission (`prism-sender`)

Handles output and data transmission:

- **Output Formats**: JSON, CSV, and custom format support
- **Destinations**: File, network, and streaming outputs
- **Buffering**: Efficient data buffering and batching
- **Reliability**: Error handling and retry mechanisms

## Metric Collection Modules

### CPU Module (`prism-cpu`)

Collects CPU-related metrics from `/proc/stat`:

**Data Sources:**
- `/proc/stat` - CPU time statistics
<!-- - `/proc/loadavg` - System load averages
- `/proc/uptime` - System uptime information -->

### Memory Module (`prism-memory`)

Collects memory information:

**Data Sources:**
- `/proc/meminfo` - Memory usage information
- `/proc/vmstat` - Virtual memory statistics
<!-- - `/proc/swaps` - Swap space information -->

### Disk Module (`prism-disk`)

Monitors disk I/O metrics:

**Data Sources:**
- `/proc/diskstats` - Disk I/O statistics
<!-- - `/proc/mounts` - Mounted filesystems
- `/proc/partitions` - Partition information -->

### Network Module (`prism-network`)

Collects network interface statistics:

**Data Sources:**
- `/proc/net/dev` - Network interface statistics
<!-- - `/proc/net/tcp` - TCP connection information
- `/proc/net/udp` - UDP socket information -->

## Data Flow

### Collection Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌──────────────┐
│   Trigger   │───▶│  Collector  │───▶│  Processor  │───▶│   Output     │
│  (Timer)    │    │  (Module)   │    │ (Transform) │    │ (Encoder)    │
└─────────────┘    └─────────────┘    └─────────────┘    └──────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌──────────────┐
│ Scheduler   │    │   procfs    │    │ Validation  │    │   Sender     │
│ Events      │    │   Files     │    │ & Cleanup   │    │ (Destination)│
└─────────────┘    └─────────────┘    └─────────────┘    └──────────────┘
```

### Event Flow

1. **Trigger Event**: Scheduler emits collection trigger
2. **Collection**: Modules read system data sources
3. **Processing**: Data is validated, transformed, and enriched
4. **Event Emission**: Processed data is emitted as events
5. **Output**: Encoders format data and senders transmit it

## Performance Characteristics

### Memory Usage

- **Low Baseline**: Minimal memory footprint when idle
- **Bounded Growth**: Memory usage scales predictably with enabled modules
- **Efficient Structures**: Zero-copy operations where possible
- **Resource Cleanup**: Automatic cleanup of temporary resources

### CPU Overhead

- **Minimal Impact**: Designed to have negligible impact on system performance
- **Efficient Parsing**: Optimized procfs parsing with minimal allocations
- **Async Operations**: Non-blocking I/O prevents system interference
- **Configurable Intervals**: Adjustable collection frequency

## Extensibility

### Plugin Architecture

The system supports custom plugins and extensions:

- **Custom Metrics**: Add new metric collection modules
- **Output Formats**: Implement custom data encoders
- **Destinations**: Create new output destinations
- **Processing**: Add custom data processing steps

### Configuration System

Hierarchical configuration with validation and hot reload capabilities:

- **TOML Format**: Human-readable configuration files
- **Environment Override**: Environment variables can override settings
- **Validation**: All configuration is validated at startup
- **Hot Reload**: Configuration changes without restart

For implementation details and development guidelines, see the [Development](./development.md) guide.

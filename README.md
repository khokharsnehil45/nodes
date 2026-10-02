# nodes (v1)

A CLI-based node-to-node system designing tool built in Rust, engineered for AI agents and developers to design system architectures, contracts, and data flows.

## Installation

The binary is compiled and installed directly to `~/.cargo/bin/nodes`:

```bash
nodes --help
```

## Quick Start

### 1. Create Nodes

```bash
# Basic node
nodes create node1 --tag "auth_service" --metadata '{"database": "postgres"}' --ninputs 2 --noutputs 1

# Node with default ports and empty metadata
nodes create gateway --tag "api_gateway" --ninputs 1 --noutputs 3
```

- `--tag`: Semantic category (e.g. `service`, `database`, `queue`, `cache`).
- `--metadata`: Arbitrary JSON payload (e.g. `'{"runtime": "rust", "port": 8080}'`).
- `--ninputs`: Number of inputs (port indices start from 0: `in:0`, `in:1`, ...).
- `--noutputs`: Number of outputs (port indices start from 0: `out:0`, `out:1`, ...).

### 2. List Nodes

```bash
# Pretty terminal table
nodes list

# Machine-readable JSON output (for AI agents & scripts)
nodes list --json
```

### 3. Inspect a Node

```bash
# Human-readable view with ports breakdown
nodes inspect node1

# Raw JSON output
nodes inspect node1 --json
```

### 4. Delete a Node

```bash
nodes delete node1
```

## State Storage

The system graph is persisted to `nodes.json` in the current working directory (or the nearest parent directory containing one).
You can also specify a custom path using `--file <path>`.

# nodes 🌐

**nodes** is a CLI-based node-to-node system designing tool written in Rust. It is engineered specifically for **AI agents and developers** to architect distributed systems, define data contracts, manage topologies, and decompose software architectures into discrete components.

---

## ⚡ Key Highlights

- **Graph-as-a-Command**: Run `nodes --create_graph <name>` to instantly generate a first-class terminal command for your graph (e.g. `graph1 --add ...`).
- **Agent-First Intermediate Representation (IR)**: Built-in `--json` support across all operations enables LLMs to inspect state and scaffold code deterministically.
- **Strict Port Contracts**: 0-indexed port addressing (`in:0..n-1`, `out:0..n-1`) with bounds checking, duplicate prevention, and schema validation.
- **Terminal Visualization Engine**: Run `draw` to see a live Unicode ASCII map of your pipeline stages, connected wires, and topology metrics.

---

## 🚀 Installation & Setup

Ensure you have Rust installed, then install `nodes` globally:

```bash
git clone https://github.com/khokharsnehil45/nodes.git
cd nodes
cargo install --path .
```

Verify the installation:
```bash
nodes --help
```

---

## 🛠️ Usage Guide

### 1. Create a Graph
Create a named graph. This generates a dedicated command launcher in your `$PATH`:

```bash
nodes --create_graph pipeline1
```

Now you can run `pipeline1` directly from any terminal session!

---

### 2. Add Nodes
Nodes comprise a **Tag**, arbitrary **JSON metadata**, and designated counts for **inputs** and **outputs** (0-indexed):

```bash
# Ingestion node (pure source: 0 inputs, 1 output)
pipeline1 --add ingest_svc --tag "source" --metadata '{"broker": "kafka"}' --ninputs 0 --noutputs 1

# Processing node (1 input, 2 outputs)
pipeline1 --add transform_svc --tag "processor" --metadata '{"timeout_ms": 250}' --ninputs 1 --noutputs 2

# Storage sink (1 input, 0 outputs)
pipeline1 --add analytics_db --tag "sink" --metadata '{"db": "clickhouse"}' --ninputs 1 --noutputs 0
```

---

### 3. Connect Ports (Data Flow Wiring)
Connect an output port of a source node to an input port of a destination node:

```bash
# Connect ingest_svc:out:0 -> transform_svc:in:0 with a data contract schema
pipeline1 --connect ingest_svc -o 0 -i 0 transform_svc --schema "raw_event.json"

# Connect transform_svc:out:0 -> analytics_db:in:0
pipeline1 --connect transform_svc -o 0 -i 0 analytics_db --schema "clean_event.json"
```

---

### 4. Visualize Network Topology (`draw`)
Render an ASCII / Unicode topology map with automatic pipeline staging, port wire mappings, and network metrics:

```bash
pipeline1 draw
```

```text
╔════════════════════════════════════════════════════════════════════════╗
║                      SYSTEM GRAPH TOPOLOGY MAP                         ║
╚════════════════════════════════════════════════════════════════════════╝

▶ STAGE 0: INGESTION / ROOTS
  ╭─ ● ingest_svc [source] ──────────────────────────────────────────────
  │  Inputs : (pure source / 0 inputs)
  │  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌
  │  [out:0] ───► transform_svc:in:0 [raw_event.json]
  │  Metadata: {"broker":"kafka"}
  ╰────────────────────────────────────────────────────────────────

▶ STAGE 1: INTERMEDIATE PROCESSORS
  ╭─ ● transform_svc [processor] ────────────────────────────────────────
  │  [in:0] ◄─── ingest_svc:out:0 [raw_event.json]
  │  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌
  │  [out:0] ───► analytics_db:in:0 [clean_event.json]
  │  [out:1] ───► (open)
  │  Metadata: {"timeout_ms":250}
  ╰────────────────────────────────────────────────────────────────

▶ STAGE 2: SINKS / TERMINAL
  ╭─ ● analytics_db [sink] ──────────────────────────────────────────────
  │  [in:0] ◄─── transform_svc:out:0 [clean_event.json]
  │  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌
  │  Outputs: (pure sink / 0 outputs)
  │  Metadata: {"db":"clickhouse"}
  ╰────────────────────────────────────────────────────────────────

▶ DATA FLOW PATHWAYS & CONTRACTS
   1. ingest_svc:out:0 ── [raw_event.json] ──► transform_svc:in:0
   2. transform_svc:out:0 ── [clean_event.json] ──► analytics_db:in:0

▶ TOPOLOGY METRICS
  • Nodes: 3 | Connections: 2 | Pipeline Depth: 3 stages
  • Sources: 1 | Sinks: 1 | Unconnected: 0
```

---

### 5. Inspect and Manage

```bash
# List all nodes in table view:
pipeline1 list

# Inspect a node with port breakdown & connections:
pipeline1 inspect transform_svc

# List all active edges:
pipeline1 edges

# Delete a node (automatically cleans up all connected edges):
pipeline1 --delete ingest_svc

# Disconnect a specific port pair:
pipeline1 --disconnect transform_svc -o 0 -i 0 analytics_db
```

---

### 6. AI Agent Integration (`--json`)

Every read command outputs structured JSON for LLMs, subagents, and automated pipelines:

```bash
# Dump complete graph IR:
pipeline1 list --json

# Inspect single node contracts:
pipeline1 inspect transform_svc --json

# Read edge contracts:
pipeline1 edges --json
```

---

## 🤖 Why It Helps AI Agents

1. **Persistent Memory**: Eliminates context degradation by externalizing architecture state to disk.
2. **Contract Enforcement**: Enforces data schemas on edges before any code is generated.
3. **Multi-Agent Task Decomposition**: An architect agent can design the graph, then assign each node to a worker subagent to implement in parallel.
4. **Zero-Hallucination Scaffolding**: Code generators build interfaces strictly against known port inputs and outputs.

---

## 📜 License

MIT License. Designed and built with Rust.

use crate::model::Graph;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_GRAPH_FILENAME: &str = "nodes.json";

pub struct Storage;

impl Storage {
    /// Finds the nearest `nodes.json` by searching upwards from current directory,
    /// or defaults to `nodes.json` in the current working directory.
    pub fn find_or_default_path() -> Result<PathBuf> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let mut cur = current_dir.as_path();

        loop {
            let candidate = cur.join(DEFAULT_GRAPH_FILENAME);
            if candidate.is_file() {
                return Ok(candidate);
            }
            match cur.parent() {
                Some(parent) => cur = parent,
                None => break,
            }
        }

        Ok(current_dir.join(DEFAULT_GRAPH_FILENAME))
    }

    /// Loads the graph from disk. If the file doesn't exist yet, returns a new blank Graph.
    pub fn load(path: &Path) -> Result<Graph> {
        if !path.exists() {
            return Ok(Graph::new());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read graph file at {}", path.display()))?;

        if content.trim().is_empty() {
            return Ok(Graph::new());
        }

        let graph: Graph = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

        Ok(graph)
    }

    /// Saves the graph to disk with formatted JSON.
    pub fn save(path: &Path, graph: &Graph) -> Result<()> {
        let serialized = serde_json::to_string_pretty(graph)
            .context("Failed to serialize graph to JSON")?;

        fs::write(path, serialized)
            .with_context(|| format!("Failed to write graph file at {}", path.display()))?;

        Ok(())
    }
}

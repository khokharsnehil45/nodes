use crate::model::Graph;
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub const DEFAULT_GRAPH_FILENAME: &str = "nodes.json";

pub struct Storage;

impl Storage {
    pub fn home_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/kevin".to_string());
        PathBuf::from(home)
    }

    pub fn global_nodes_dir() -> PathBuf {
        Self::home_dir().join(".nodes")
    }

    pub fn global_registry_path() -> PathBuf {
        Self::global_nodes_dir().join("registry.json")
    }

    pub fn cargo_bin_dir() -> PathBuf {
        Self::home_dir().join(".cargo").join("bin")
    }

    /// Read global registry of graphs
    pub fn read_registry() -> BTreeMap<String, String> {
        let reg_path = Self::global_registry_path();
        if let Ok(content) = fs::read_to_string(&reg_path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            BTreeMap::new()
        }
    }

    /// Update global registry with a graph path
    pub fn register_graph(name: &str, path: &Path) -> Result<()> {
        let mut reg = Self::read_registry();
        reg.insert(name.to_string(), path.to_string_lossy().to_string());
        let reg_dir = Self::global_nodes_dir();
        fs::create_dir_all(&reg_dir)?;
        let serialized = serde_json::to_string_pretty(&reg)?;
        fs::write(Self::global_registry_path(), serialized)?;
        Ok(())
    }

    /// Remove a graph from registry
    pub fn unregister_graph(name: &str) -> Result<()> {
        let mut reg = Self::read_registry();
        reg.remove(name);
        let serialized = serde_json::to_string_pretty(&reg)?;
        fs::write(Self::global_registry_path(), serialized)?;
        Ok(())
    }

    /// Creates a dedicated launcher shell script in ~/.cargo/bin/<graph_name>
    pub fn create_launcher(name: &str) -> Result<PathBuf> {
        let bin_dir = Self::cargo_bin_dir();
        fs::create_dir_all(&bin_dir)
            .with_context(|| format!("Failed to create bin dir: {}", bin_dir.display()))?;

        let launcher_path = bin_dir.join(name);
        let script_content = format!(
            "#!/bin/sh\n# Auto-generated launcher for graph '{}'\nexec nodes --graph \"{}\" \"$@\"\n",
            name, name
        );

        fs::write(&launcher_path, script_content)
            .with_context(|| format!("Failed to write launcher script at {}", launcher_path.display()))?;

        // Make executable (chmod 755)
        let mut perms = fs::metadata(&launcher_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&launcher_path, perms)?;

        Ok(launcher_path)
    }

    /// Remove launcher if exists
    pub fn remove_launcher(name: &str) {
        let launcher_path = Self::cargo_bin_dir().join(name);
        let _ = fs::remove_file(launcher_path);
    }

    /// Initializes a new named graph, creates local storage and ~/.cargo/bin command
    pub fn create_graph(name: &str) -> Result<(PathBuf, PathBuf)> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let local_nodes_dir = current_dir.join(".nodes");
        fs::create_dir_all(&local_nodes_dir)?;

        let graph_file = local_nodes_dir.join(format!("{}.json", name));
        if graph_file.exists() {
            bail!("Graph '{}' already exists at {}", name, graph_file.display());
        }

        let new_graph = Graph::new();
        Self::save(&graph_file, &new_graph)?;
        Self::register_graph(name, &graph_file)?;
        let launcher = Self::create_launcher(name)?;

        Ok((graph_file, launcher))
    }

    /// Resolves the file path for a graph
    pub fn resolve_graph_path(graph_name: Option<&str>) -> Result<PathBuf> {
        match graph_name {
            Some(name) => {
                let current_dir = std::env::current_dir().context("Failed to get current directory")?;
                
                // 1. Check local .nodes/<name>.json in current dir or parents
                let mut cur = current_dir.as_path();
                loop {
                    let local_file = cur.join(".nodes").join(format!("{}.json", name));
                    if local_file.is_file() {
                        return Ok(local_file);
                    }
                    let direct_file = cur.join(format!("{}.json", name));
                    if direct_file.is_file() {
                        return Ok(direct_file);
                    }
                    match cur.parent() {
                        Some(p) => cur = p,
                        None => break,
                    }
                }

                // 2. Check global registry
                let reg = Self::read_registry();
                if let Some(path_str) = reg.get(name) {
                    let p = PathBuf::from(path_str);
                    if p.is_file() {
                        return Ok(p);
                    }
                }

                // 3. Check ~/.nodes/graphs/<name>.json
                let global_file = Self::global_nodes_dir().join("graphs").join(format!("{}.json", name));
                if global_file.is_file() {
                    return Ok(global_file);
                }

                // 4. Default: local .nodes/<name>.json in CWD
                let fallback = current_dir.join(".nodes").join(format!("{}.json", name));
                Ok(fallback)
            }
            None => {
                // Single-graph default fallback
                Self::find_or_default_path()
            }
        }
    }

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
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let serialized = serde_json::to_string_pretty(graph)
            .context("Failed to serialize graph to JSON")?;

        fs::write(path, serialized)
            .with_context(|| format!("Failed to write graph file at {}", path.display()))?;

        Ok(())
    }
}

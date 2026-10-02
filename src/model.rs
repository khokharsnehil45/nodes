use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default = "default_metadata")]
    pub metadata: Value,
    #[serde(default)]
    pub n_inputs: u32,
    #[serde(default)]
    pub n_outputs: u32,
}

fn default_metadata() -> Value {
    Value::Object(serde_json::Map::new())
}

impl Node {
    pub fn new(name: String, tag: Option<String>, metadata: Value, n_inputs: u32, n_outputs: u32) -> Self {
        Self {
            name,
            tag,
            metadata,
            n_inputs,
            n_outputs,
        }
    }

    pub fn input_ports(&self) -> Vec<String> {
        (0..self.n_inputs).map(|i| format!("in:{}", i)).collect()
    }

    pub fn output_ports(&self) -> Vec<String> {
        (0..self.n_outputs).map(|i| format!("out:{}", i)).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Edge {
    pub from_node: String,
    pub from_port: u32,
    pub to_node: String,
    pub to_port: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Graph {
    pub version: String,
    #[serde(default)]
    pub nodes: BTreeMap<String, Node>,
    #[serde(default)]
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            nodes: BTreeMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), String> {
        if self.nodes.contains_key(&node.name) {
            return Err(format!("Node '{}' already exists", node.name));
        }
        self.nodes.insert(node.name.clone(), node);
        Ok(())
    }

    pub fn remove_node(&mut self, name: &str) -> Option<Node> {
        // Also remove any dangling edges connected to this node
        self.edges.retain(|e| e.from_node != name && e.to_node != name);
        self.nodes.remove(name)
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.get(name)
    }
}

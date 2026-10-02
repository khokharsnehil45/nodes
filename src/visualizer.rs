use crate::model::Graph;
use colored::*;
use std::collections::{BTreeMap, HashSet, VecDeque};

pub struct Visualizer;

impl Visualizer {
    pub fn render(graph: &Graph) {
        if graph.nodes.is_empty() {
            println!("{}", "Graph is empty. No nodes to visualize.".yellow());
            return;
        }

        println!("\n{}", "╔════════════════════════════════════════════════════════════════════════╗".cyan().bold());
        println!("{}", "║                      SYSTEM GRAPH TOPOLOGY MAP                         ║".cyan().bold());
        println!("{}\n", "╚════════════════════════════════════════════════════════════════════════╝".cyan().bold());

        // 1. Compute Stages / Layers using topological sorting
        let layers = Self::compute_layers(graph);

        for (stage_idx, layer_nodes) in layers.iter().enumerate() {
            let stage_title = if stage_idx == 0 {
                "STAGE 0: INGESTION / ROOTS".to_string()
            } else if stage_idx == layers.len() - 1 && layers.len() > 1 {
                format!("STAGE {}: SINKS / TERMINAL", stage_idx)
            } else {
                format!("STAGE {}: INTERMEDIATE PROCESSORS", stage_idx)
            };

            println!(
                "{} {}",
                "▶".yellow().bold(),
                stage_title.bold().underline()
            );

            for node_name in layer_nodes {
                if let Some(node) = graph.get_node(node_name) {
                    Self::render_node_card(graph, node);
                }
            }
            println!();
        }

        // 2. Render Active Wire / Data Flow Pathways
        println!("{}", "▶ DATA FLOW PATHWAYS & CONTRACTS".yellow().bold().underline());
        if graph.edges.is_empty() {
            println!("  {}", "(No active connections between nodes)".dimmed());
        } else {
            for (i, edge) in graph.edges.iter().enumerate() {
                let schema_str = match &edge.schema {
                    Some(s) => format!("── [{}] ──", s.yellow()),
                    None => "──────────".to_string(),
                };

                println!(
                    "  {:2}. {}:{} {}► {}:{}",
                    i + 1,
                    edge.from_node.cyan().bold(),
                    format!("out:{}", edge.from_port).magenta(),
                    schema_str,
                    edge.to_node.cyan().bold(),
                    format!("in:{}", edge.to_port).green()
                );
            }
        }

        // 3. Topology Health & Summary Metrics
        println!("\n{}", "▶ TOPOLOGY METRICS".yellow().bold().underline());
        let total_nodes = graph.nodes.len();
        let total_edges = graph.edges.len();

        let mut isolated = 0;
        let mut pure_sources = 0;
        let mut pure_sinks = 0;

        for (name, _) in &graph.nodes {
            let in_cnt = graph.incoming_edges(name).len();
            let out_cnt = graph.outgoing_edges(name).len();

            if in_cnt == 0 && out_cnt == 0 {
                isolated += 1;
            } else if in_cnt == 0 && out_cnt > 0 {
                pure_sources += 1;
            } else if in_cnt > 0 && out_cnt == 0 {
                pure_sinks += 1;
            }
        }

        println!(
            "  • Nodes: {} | Connections: {} | Pipeline Depth: {} stages",
            total_nodes.to_string().cyan().bold(),
            total_edges.to_string().green().bold(),
            layers.len().to_string().yellow().bold()
        );
        println!(
            "  • Sources: {} | Sinks: {} | Unconnected: {}",
            pure_sources.to_string().cyan(),
            pure_sinks.to_string().green(),
            if isolated > 0 {
                isolated.to_string().red().bold()
            } else {
                "0".dimmed()
            }
        );
        println!();
    }

    fn render_node_card(graph: &Graph, node: &crate::model::Node) {
        let tag_label = match &node.tag {
            Some(t) => format!(" [{}]", t.yellow()),
            None => "".to_string(),
        };

        let width: usize = 64;
        let title_line = format!("● {}{}", node.name.cyan().bold(), tag_label);
        
        println!("  ╭─ {} {}", title_line, "─".repeat(width.saturating_sub(node.name.len() + 10)).dimmed());

        // Render Inputs
        if node.n_inputs == 0 {
            println!("  │  Inputs : (pure source / 0 inputs)");
        } else {
            for port in 0..node.n_inputs {
                let incoming: Vec<_> = graph
                    .edges
                    .iter()
                    .filter(|e| e.to_node == node.name && e.to_port == port)
                    .collect();

                if incoming.is_empty() {
                    println!(
                        "  │  [in:{}] ◄─── (unconnected)",
                        format!("{}", port).green()
                    );
                } else {
                    for edge in incoming {
                        let schema_tag = match &edge.schema {
                            Some(s) => format!(" [{}]", s.dimmed()),
                            None => String::new(),
                        };
                        println!(
                            "  │  [in:{}] ◄─── {}:out:{}{}",
                            format!("{}", port).green().bold(),
                            edge.from_node.cyan(),
                            edge.from_port,
                            schema_tag
                        );
                    }
                }
            }
        }

        println!("  │  {}", "╌".repeat(width - 4).dimmed());

        // Render Outputs
        if node.n_outputs == 0 {
            println!("  │  Outputs: (pure sink / 0 outputs)");
        } else {
            for port in 0..node.n_outputs {
                let outgoing: Vec<_> = graph
                    .edges
                    .iter()
                    .filter(|e| e.from_node == node.name && e.from_port == port)
                    .collect();

                if outgoing.is_empty() {
                    println!(
                        "  │  [out:{}] ───► (open)",
                        format!("{}", port).magenta()
                    );
                } else {
                    for edge in outgoing {
                        let schema_tag = match &edge.schema {
                            Some(s) => format!(" [{}]", s.dimmed()),
                            None => String::new(),
                        };
                        println!(
                            "  │  [out:{}] ───► {}:in:{}{}",
                            format!("{}", port).magenta().bold(),
                            edge.to_node.cyan(),
                            edge.to_port,
                            schema_tag
                        );
                    }
                }
            }
        }

        // Render Metadata snippet if not empty
        if !node.metadata.as_object().map_or(true, |m| m.is_empty()) {
            let meta_json = serde_json::to_string(&node.metadata).unwrap_or_default();
            let truncated_meta = if meta_json.len() > 50 {
                format!("{}...", &meta_json[..47])
            } else {
                meta_json
            };
            println!("  │  Metadata: {}", truncated_meta.dimmed());
        }

        println!("  ╰{}", "─".repeat(width).dimmed());
    }

    /// Computes topological layers for DAG representation
    fn compute_layers(graph: &Graph) -> Vec<Vec<String>> {
        let mut in_degrees: BTreeMap<String, usize> = BTreeMap::new();
        let mut adj_list: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for name in graph.nodes.keys() {
            in_degrees.insert(name.clone(), 0);
            adj_list.insert(name.clone(), Vec::new());
        }

        for edge in &graph.edges {
            if in_degrees.contains_key(&edge.to_node) && adj_list.contains_key(&edge.from_node) {
                *in_degrees.entry(edge.to_node.clone()).or_insert(0) += 1;
                adj_list.entry(edge.from_node.clone()).or_default().push(edge.to_node.clone());
            }
        }

        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        let mut visited = HashSet::new();

        // Nodes with in_degree == 0 start at layer 0
        for (name, deg) in &in_degrees {
            if *deg == 0 {
                queue.push_back((name.clone(), 0));
                visited.insert(name.clone());
            }
        }

        // If no pure source exists (e.g. fully cyclic graph), seed with the first node
        if queue.is_empty() {
            if let Some(first_node) = graph.nodes.keys().next() {
                queue.push_back((first_node.clone(), 0));
                visited.insert(first_node.clone());
            }
        }

        let mut node_layers: BTreeMap<String, usize> = BTreeMap::new();

        while let Some((node, layer)) = queue.pop_front() {
            node_layers.insert(node.clone(), layer);

            if let Some(neighbors) = adj_list.get(&node) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        queue.push_back((neighbor.clone(), layer + 1));
                    }
                }
            }
        }

        // Add any remaining unvisited nodes
        for name in graph.nodes.keys() {
            if !node_layers.contains_key(name) {
                node_layers.insert(name.clone(), 0);
            }
        }

        // Group nodes by layer index
        let mut max_layer = 0;
        for &layer in node_layers.values() {
            if layer > max_layer {
                max_layer = layer;
            }
        }

        let mut layers: Vec<Vec<String>> = vec![Vec::new(); max_layer + 1];
        for (name, layer) in node_layers {
            layers[layer].push(name);
        }

        // Filter out empty layers
        layers.into_iter().filter(|l| !l.is_empty()).collect()
    }
}

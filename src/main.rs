mod cli;
mod model;
mod storage;

use anyhow::{bail, Context, Result};
use clap::Parser;
use cli::{Cli, Commands, ConnectArgs, CreateArgs, DeleteArgs, DisconnectArgs, InspectArgs, ListArgs};
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use model::{Edge, Node};
use storage::Storage;

fn main() -> Result<()> {
    // Normalize command-line arguments to seamlessly support `--connect`, `-connect`, `-create`, etc.
    let raw_args: Vec<String> = std::env::args().collect();
    let normalized_args: Vec<String> = raw_args
        .into_iter()
        .enumerate()
        .map(|(idx, arg)| {
            if idx == 1 {
                match arg.as_str() {
                    "--connect" | "-connect" => "connect".to_string(),
                    "--create" | "-create" => "create".to_string(),
                    "--list" | "-list" => "list".to_string(),
                    "--inspect" | "-inspect" => "inspect".to_string(),
                    "--delete" | "-delete" => "delete".to_string(),
                    "--disconnect" | "-disconnect" => "disconnect".to_string(),
                    "--edges" | "-edges" => "edges".to_string(),
                    _ => arg,
                }
            } else {
                arg
            }
        })
        .collect();

    let cli = Cli::parse_from(normalized_args);

    let graph_path = match cli.file {
        Some(path) => path,
        None => Storage::find_or_default_path()?,
    };

    match cli.command {
        Commands::Init => {
            if graph_path.exists() {
                println!(
                    "{} Graph file already exists at {}",
                    "ℹ".yellow().bold(),
                    graph_path.display().to_string().cyan()
                );
            } else {
                let empty_graph = model::Graph::new();
                Storage::save(&graph_path, &empty_graph)?;
                println!(
                    "{} Initialized new nodes graph at {}",
                    "✓".green().bold(),
                    graph_path.display().to_string().cyan()
                );
            }
        }

        Commands::Create(args) => {
            handle_create(&graph_path, args)?;
        }

        Commands::List(args) => {
            handle_list(&graph_path, args)?;
        }

        Commands::Inspect(args) => {
            handle_inspect(&graph_path, args)?;
        }

        Commands::Delete(args) => {
            handle_delete(&graph_path, args)?;
        }

        Commands::Connect(args) => {
            handle_connect(&graph_path, args)?;
        }

        Commands::Disconnect(args) => {
            handle_disconnect(&graph_path, args)?;
        }

        Commands::Edges(args) => {
            handle_edges(&graph_path, args)?;
        }
    }

    Ok(())
}

fn handle_create(path: &std::path::Path, args: CreateArgs) -> Result<()> {
    let mut graph = Storage::load(path)?;

    // Parse and validate metadata JSON
    let metadata_val: serde_json::Value = serde_json::from_str(&args.metadata)
        .with_context(|| format!("Invalid JSON passed to --metadata: '{}'", args.metadata))?;

    let node = Node::new(
        args.name.clone(),
        args.tag.clone(),
        metadata_val,
        args.ninputs,
        args.noutputs,
    );

    if let Err(err) = graph.add_node(node.clone()) {
        bail!("{}: {}", "Error".red().bold(), err);
    }

    Storage::save(path, &graph)?;

    println!(
        "{} Created node '{}' [tag: {}, inputs: 0..{}, outputs: 0..{}]",
        "✓".green().bold(),
        node.name.bold().cyan(),
        node.tag.as_deref().unwrap_or("-").yellow(),
        if node.n_inputs > 0 { node.n_inputs - 1 } else { 0 },
        if node.n_outputs > 0 { node.n_outputs - 1 } else { 0 },
    );
    println!("  Saved to: {}", path.display().to_string().dimmed());

    Ok(())
}

fn handle_connect(path: &std::path::Path, args: ConnectArgs) -> Result<()> {
    let mut graph = Storage::load(path)?;

    let edge = Edge {
        from_node: args.from_node.clone(),
        from_port: args.out_port,
        to_node: args.to_node.clone(),
        to_port: args.in_port,
        schema: args.schema.clone(),
    };

    if let Err(err) = graph.add_edge(edge.clone()) {
        bail!("{}: {}", "Connection Error".red().bold(), err);
    }

    Storage::save(path, &graph)?;

    let schema_text = match &edge.schema {
        Some(s) => format!(" [schema: {}]", s.dimmed()),
        None => String::new(),
    };

    println!(
        "{} Connected {}:{} ──► {}:{}{}",
        "✓".green().bold(),
        edge.from_node.cyan().bold(),
        format!("out:{}", edge.from_port).magenta().bold(),
        edge.to_node.cyan().bold(),
        format!("in:{}", edge.to_port).green().bold(),
        schema_text
    );
    println!("  Graph file: {}", path.display().to_string().dimmed());

    Ok(())
}

fn handle_disconnect(path: &std::path::Path, args: DisconnectArgs) -> Result<()> {
    let mut graph = Storage::load(path)?;

    let initial_len = graph.edges.len();
    graph.edges.retain(|e| {
        !(e.from_node == args.from_node
            && e.from_port == args.out_port
            && e.to_node == args.to_node
            && e.to_port == args.in_port)
    });

    if graph.edges.len() == initial_len {
        bail!(
            "No connection found matching {}:out:{} -> {}:in:{}",
            args.from_node,
            args.out_port,
            args.to_node,
            args.in_port
        );
    }

    Storage::save(path, &graph)?;

    println!(
        "{} Disconnected {}:out:{} ──x──► {}:in:{}",
        "✓".green().bold(),
        args.from_node.cyan(),
        args.out_port,
        args.to_node.cyan(),
        args.in_port
    );

    Ok(())
}

fn handle_edges(path: &std::path::Path, args: ListArgs) -> Result<()> {
    let graph = Storage::load(path)?;

    if args.json {
        let json_output = serde_json::to_string_pretty(&graph.edges)?;
        println!("{}", json_output);
        return Ok(());
    }

    if graph.edges.is_empty() {
        println!("{}", "No connections / edges found in the system graph.".yellow());
        println!("Run 'nodes connect <from> -o <out_port> -i <in_port> <to>' to connect nodes.");
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("#").fg(Color::DarkGrey),
            Cell::new("From Node:Port").fg(Color::Magenta),
            Cell::new("Direction").fg(Color::White),
            Cell::new("To Node:Port").fg(Color::Green),
            Cell::new("Schema / Contract").fg(Color::Yellow),
        ]);

    for (idx, edge) in graph.edges.iter().enumerate() {
        let from_str = format!("{}:out:{}", edge.from_node, edge.from_port);
        let to_str = format!("{}:in:{}", edge.to_node, edge.to_port);
        let schema_str = edge.schema.as_deref().unwrap_or("-");

        table.add_row(vec![
            Cell::new((idx + 1).to_string()),
            Cell::new(from_str),
            Cell::new("──►"),
            Cell::new(to_str),
            Cell::new(schema_str),
        ]);
    }

    println!("{}", table);
    println!(
        "Total connections: {} | Graph file: {}",
        graph.edges.len().to_string().green().bold(),
        path.display().to_string().dimmed()
    );

    Ok(())
}

fn handle_list(path: &std::path::Path, args: ListArgs) -> Result<()> {
    let graph = Storage::load(path)?;

    if args.json {
        let json_output = serde_json::to_string_pretty(&graph)?;
        println!("{}", json_output);
        return Ok(());
    }

    if graph.nodes.is_empty() {
        println!("{}", "No nodes found in the current system graph.".yellow());
        println!("Run 'nodes create <name>' to create your first node.");
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Node Name").fg(Color::Cyan),
            Cell::new("Tag").fg(Color::Yellow),
            Cell::new("Inputs (ports)").fg(Color::Green),
            Cell::new("Outputs (ports)").fg(Color::Magenta),
            Cell::new("Connections").fg(Color::White),
            Cell::new("Metadata"),
        ]);

    for node in graph.nodes.values() {
        let tag_display = node.tag.as_deref().unwrap_or("-");

        let inputs_display = if node.n_inputs == 0 {
            "0".to_string()
        } else {
            format!("{} (in:0..in:{})", node.n_inputs, node.n_inputs - 1)
        };

        let outputs_display = if node.n_outputs == 0 {
            "0".to_string()
        } else {
            format!("{} (out:0..out:{})", node.n_outputs, node.n_outputs - 1)
        };

        let in_edges = graph.incoming_edges(&node.name).len();
        let out_edges = graph.outgoing_edges(&node.name).len();
        let conn_display = format!("in: {}, out: {}", in_edges, out_edges);

        let meta_display = serde_json::to_string(&node.metadata).unwrap_or_else(|_| "{}".into());

        table.add_row(vec![
            Cell::new(&node.name),
            Cell::new(tag_display),
            Cell::new(inputs_display),
            Cell::new(outputs_display),
            Cell::new(conn_display),
            Cell::new(meta_display),
        ]);
    }

    println!("{}", table);
    println!(
        "Total nodes: {} | Total connections: {} | Graph file: {}",
        graph.nodes.len().to_string().green().bold(),
        graph.edges.len().to_string().green().bold(),
        path.display().to_string().dimmed()
    );

    Ok(())
}

fn handle_inspect(path: &std::path::Path, args: InspectArgs) -> Result<()> {
    let graph = Storage::load(path)?;

    let node = match graph.get_node(&args.name) {
        Some(n) => n,
        None => bail!("Node '{}' does not exist in graph.", args.name),
    };

    let incoming = graph.incoming_edges(&node.name);
    let outgoing = graph.outgoing_edges(&node.name);

    if args.json {
        let inspection_obj = serde_json::json!({
            "node": node,
            "incoming_edges": incoming,
            "outgoing_edges": outgoing,
        });
        println!("{}", serde_json::to_string_pretty(&inspection_obj)?);
        return Ok(());
    }

    println!("\n{}", "================ NODE SPECIFICATION ================".cyan().bold());
    println!("{}: {}", "Name".bold(), node.name.cyan().bold());
    println!("{}: {}", "Tag".bold(), node.tag.as_deref().unwrap_or("-").yellow());
    println!(
        "{}: {} {}",
        "Inputs".bold(),
        node.n_inputs,
        format!("({:?})", node.input_ports()).dimmed()
    );
    println!(
        "{}: {} {}",
        "Outputs".bold(),
        node.n_outputs,
        format!("({:?})", node.output_ports()).dimmed()
    );

    println!("\n{}", "--- CONNECTIONS ---".dimmed());
    if incoming.is_empty() {
        println!("{}: (none)", "Incoming".bold());
    } else {
        println!("{}:", "Incoming".bold());
        for edge in incoming {
            println!(
                "  • {}:out:{} ──► {}:in:{}",
                edge.from_node.cyan(),
                edge.from_port,
                edge.to_node.cyan(),
                edge.to_port
            );
        }
    }

    if outgoing.is_empty() {
        println!("{}: (none)", "Outgoing".bold());
    } else {
        println!("{}:", "Outgoing".bold());
        for edge in outgoing {
            println!(
                "  • {}:out:{} ──► {}:in:{}",
                edge.from_node.cyan(),
                edge.from_port,
                edge.to_node.cyan(),
                edge.to_port
            );
        }
    }

    println!("\n{}:", "Metadata".bold());
    println!("{}", serde_json::to_string_pretty(&node.metadata)?);
    println!("{}\n", "===================================================".cyan().bold());

    Ok(())
}

fn handle_delete(path: &std::path::Path, args: DeleteArgs) -> Result<()> {
    let mut graph = Storage::load(path)?;

    match graph.remove_node(&args.name) {
        Some(_) => {
            Storage::save(path, &graph)?;
            println!("{} Deleted node '{}'", "✓".green().bold(), args.name.cyan());
            Ok(())
        }
        None => {
            bail!("Node '{}' does not exist in graph.", args.name);
        }
    }
}

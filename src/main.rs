mod cli;
mod model;
mod storage;

use anyhow::{bail, Context, Result};
use clap::Parser;
use cli::{Cli, Commands, CreateArgs, DeleteArgs, InspectArgs, ListArgs};
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use model::Node;
use storage::Storage;

fn main() -> Result<()> {
    let cli = Cli::parse();

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

fn handle_list(path: &std::path::Path, args: ListArgs) -> Result<()> {
    let graph = Storage::load(path)?;

    if args.json {
        let json_output = serde_json::to_string_pretty(&graph.nodes.values().collect::<Vec<_>>())?;
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

        let meta_display = serde_json::to_string(&node.metadata).unwrap_or_else(|_| "{}".into());

        table.add_row(vec![
            Cell::new(&node.name),
            Cell::new(tag_display),
            Cell::new(inputs_display),
            Cell::new(outputs_display),
            Cell::new(meta_display),
        ]);
    }

    println!("{}", table);
    println!(
        "Total nodes: {} | Graph file: {}",
        graph.nodes.len().to_string().green().bold(),
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

    if args.json {
        let json_output = serde_json::to_string_pretty(node)?;
        println!("{}", json_output);
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
    println!("{}:", "Metadata".bold());
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

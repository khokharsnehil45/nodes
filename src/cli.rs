use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nodes")]
#[command(about = "CLI-based node-to-node system designing tool for AI agents and developers", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Target graph name (e.g. graph1)
    #[arg(long, global = true)]
    pub graph: Option<String>,

    /// Custom path to the nodes.json graph file
    #[arg(long, global = true)]
    pub file: Option<std::path::PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new named graph and generate its standalone CLI command (e.g. graph1)
    #[command(
        alias = "create_graph",
        alias = "create-graph",
        alias = "--create_graph",
        alias = "--create-graph"
    )]
    CreateGraph(CreateGraphArgs),

    /// List all registered graphs
    #[command(alias = "graphs", alias = "list_graphs", alias = "list-graphs")]
    ListGraphs(ListArgs),

    /// Delete a named graph and remove its CLI command
    #[command(alias = "delete_graph", alias = "delete-graph", alias = "rm-graph")]
    DeleteGraph(DeleteGraphArgs),

    /// Add a new node to the system graph
    #[command(
        alias = "--add",
        alias = "-add",
        alias = "add",
        alias = "--create",
        alias = "-create",
        alias = "create",
        alias = "new"
    )]
    Add(AddArgs),

    /// List all nodes in the system graph
    #[command(alias = "--list", alias = "-list", alias = "ls")]
    List(ListArgs),

    /// Inspect details of a specific node
    #[command(alias = "--inspect", alias = "-inspect", alias = "get", alias = "show")]
    Inspect(InspectArgs),

    /// Delete a node from the system graph
    #[command(alias = "--delete", alias = "-delete", alias = "rm", alias = "remove")]
    Delete(DeleteArgs),

    /// Connect an output port of a node to an input port of another node
    #[command(alias = "--connect", alias = "-connect", alias = "link")]
    Connect(ConnectArgs),

    /// Disconnect nodes
    #[command(alias = "--disconnect", alias = "-disconnect", alias = "unlink")]
    Disconnect(DisconnectArgs),

    /// List all connections / edges in the system graph
    #[command(alias = "--edges", alias = "-edges", alias = "connections", alias = "links")]
    Edges(ListArgs),

    /// Visualize the graph network topology in the terminal
    #[command(alias = "render", alias = "view", alias = "--draw", alias = "--render", alias = "--view")]
    Draw,

    /// Initialize a new empty nodes.json graph in current directory
    Init,
}

#[derive(Args, Debug)]
pub struct CreateGraphArgs {
    /// Name of the graph (e.g. graph1, order_engine)
    pub name: String,
}

#[derive(Args, Debug)]
pub struct DeleteGraphArgs {
    /// Name of the graph to delete
    pub name: String,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Name or unique identifier of the node (e.g. node1, auth-service)
    pub name: String,

    /// Semantic tag or category for the node (e.g. service, database, queue)
    #[arg(long)]
    pub tag: Option<String>,

    /// Metadata in JSON format (e.g. '{"lang": "rust", "port": 8080}')
    #[arg(long, default_value = "{}")]
    pub metadata: String,

    /// Number of inputs (ports indexed from 0 to n_inputs-1)
    #[arg(long, alias = "inputs", default_value_t = 1)]
    pub ninputs: u32,

    /// Number of outputs (ports indexed from 0 to n_outputs-1)
    #[arg(long, alias = "outputs", default_value_t = 1)]
    pub noutputs: u32,
}

#[derive(Args, Debug)]
pub struct ConnectArgs {
    /// Source node name
    pub from_node: String,

    /// Destination node name
    pub to_node: String,

    /// Output port index of the source node (default: 0)
    #[arg(short = 'o', long = "out", alias = "from-port", alias = "output", default_value_t = 0)]
    pub out_port: u32,

    /// Input port index of the destination node (default: 0)
    #[arg(short = 'i', long = "in", alias = "to-port", alias = "input", default_value_t = 0)]
    pub in_port: u32,

    /// Optional schema or contract for this connection (e.g. 'auth.schema.json')
    #[arg(long)]
    pub schema: Option<String>,
}

#[derive(Args, Debug)]
pub struct DisconnectArgs {
    /// Source node name
    pub from_node: String,

    /// Destination node name
    pub to_node: String,

    /// Output port index of the source node (default: 0)
    #[arg(short = 'o', long = "out", default_value_t = 0)]
    pub out_port: u32,

    /// Input port index of the destination node (default: 0)
    #[arg(short = 'i', long = "in", default_value_t = 0)]
    pub in_port: u32,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Output raw JSON format (ideal for AI agents and scripts)
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct InspectArgs {
    /// Name of the node to inspect
    pub name: String,

    /// Output raw JSON format
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// Name of the node to delete
    pub name: String,
}

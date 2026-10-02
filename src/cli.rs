use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nodes")]
#[command(about = "CLI-based node-to-node system designing tool for AI agents and developers", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Custom path to the nodes.json graph file
    #[arg(long, global = true)]
    pub file: Option<std::path::PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new node in the system graph
    #[command(alias = "-create", alias = "add", alias = "new")]
    Create(CreateArgs),

    /// List all nodes in the system graph
    #[command(alias = "ls")]
    List(ListArgs),

    /// Inspect details of a specific node
    #[command(alias = "get", alias = "show")]
    Inspect(InspectArgs),

    /// Delete a node from the system graph
    #[command(alias = "rm", alias = "remove")]
    Delete(DeleteArgs),

    /// Connect an output port of a node to an input port of another node
    #[command(alias = "--connect", alias = "-connect", alias = "link")]
    Connect(ConnectArgs),

    /// Disconnect nodes
    #[command(alias = "--disconnect", alias = "-disconnect", alias = "unlink")]
    Disconnect(DisconnectArgs),

    /// List all connections / edges in the system graph
    #[command(alias = "connections", alias = "links")]
    Edges(ListArgs),

    /// Initialize a new empty nodes.json graph in current directory
    Init,
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
pub struct CreateArgs {
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

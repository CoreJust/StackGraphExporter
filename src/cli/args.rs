use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "stackgraph_exporter",
    about = "Stack Graph Exporter and Query Tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Open(OpenArgs),
    Help,
}

#[derive(Args)]
pub struct OpenArgs {
    // Language choice
    #[arg(short, long)]
    pub java: bool,
    #[arg(short, long)]
    pub python: bool,

    // Backend choice
    #[arg(long)]
    pub kotgll: bool,
    #[arg(long)]
    pub ucfs: bool,

    #[arg(short, long)]
    pub query: bool,
    #[arg(long)]
    pub verify: bool,
    #[arg(short, long)]
    pub verbose: bool,

    // Show all references, not only those with at least one partial path
    #[arg(long, alias = "all")]
    pub all_symbols: bool,

    #[arg(long, alias = "simplify")]
    pub simplify_cfl: bool,

    // For KotGLL only
    #[arg(long)]
    pub sppf: bool,
    #[arg(long)]
    pub kotgll_path: Option<PathBuf>,

    // Artifacts generation
    #[arg(long)]
    pub cfg: bool,
    #[arg(long)]
    pub csv: bool,
    #[arg(long)]
    pub dot: bool,
    #[arg(long)]
    pub dot_ucfs: bool,
    #[arg(long)]
    pub kt: bool,
    #[arg(long, alias = "sg-json", alias = "json")]
    pub stack_graph_json: bool,

    // Output paths
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub output_cfg: Option<PathBuf>,
    #[arg(long)]
    pub output_csv: Option<PathBuf>,
    #[arg(long)]
    pub output_dot: Option<PathBuf>,
    #[arg(long)]
    pub output_dot_ucfs: Option<PathBuf>,
    #[arg(long)]
    pub output_kt: Option<PathBuf>,
    #[arg(long)]
    pub output_stack_graph_json: Option<PathBuf>,

    // Immediate queries
    #[arg(short = 's', long)]
    pub symbol: Option<String>,
    #[arg(long)]
    pub source: Option<String>,

    // Path to the project root
    pub path: PathBuf,
}

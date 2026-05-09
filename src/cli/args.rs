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
}

#[derive(Args)]
pub struct OpenArgs {
    // Loading flags
    #[arg(long)]
    pub remove_unsupported: bool,

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
    #[arg(long)]
    pub cfg_bench: bool,

    #[arg(short, long)]
    pub verbose: bool,
    #[arg(long)]
    pub verify: bool,

    // Show all references, not only those with at least one partial path
    #[arg(long, alias = "all")]
    pub all_symbols: bool,

    #[arg(long)]
    pub simplify: bool,
    #[arg(long, alias = "max-simplify-iterations")]
    pub max_simplification_iterations: Option<usize>,
    #[arg(long, alias = "eps-tolerance")]
    pub eps_removal_tolerance: Option<isize>,
    #[arg(long)]
    pub remove_unreachable_trivial: bool,
    #[arg(long)]
    pub remove_unreachable: bool,
    #[arg(long, alias = "with-front")]
    pub remove_unreachable_with_front: bool,
    #[arg(long)]
    pub remove_unreachable_deep: Option<u8>,

    #[arg(long)]
    pub inverse: bool,

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
    #[arg(long, alias = "sg-dot")]
    pub stack_graph_dot: bool,
    #[arg(long)]
    pub dot_ucfs: bool,
    #[arg(long)]
    pub kt: bool,
    #[arg(long, alias = "sg-json")]
    pub stack_graph_json: bool,
    #[arg(long)]
    pub g: bool,
    #[arg(long)]
    pub g_cfg: bool,
    #[arg(long)]
    pub cnf: bool,
    #[arg(long)]
    pub cnf_cfg: bool,

    // Output paths
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub output_cfg: Option<PathBuf>,
    #[arg(long)]
    pub output_csv: Option<PathBuf>,
    #[arg(long, alias = "output-sg-dot")]
    pub output_stack_graph_dot: Option<PathBuf>,
    #[arg(long)]
    pub output_dot_ucfs: Option<PathBuf>,
    #[arg(long)]
    pub output_kt: Option<PathBuf>,
    #[arg(long, alias = "output-sg-json")]
    pub output_stack_graph_json: Option<PathBuf>,
    #[arg(long)]
    pub output_g: Option<PathBuf>,
    #[arg(long)]
    pub output_g_cfg: Option<PathBuf>,
    #[arg(long)]
    pub output_cnf: Option<PathBuf>,
    #[arg(long)]
    pub output_cnf_cfg: Option<PathBuf>,

    // Immediate queries
    #[arg(short = 's', long)]
    pub symbol: Option<String>,
    #[arg(long)]
    pub source: Option<String>,
    #[arg(long)]
    pub pick_queries: Option<u32>,
    #[arg(long, alias = "all-paths", alias = "query-all")]
    pub query_all_paths: bool,
    #[arg(long)]
    pub create: bool,

    // Path to the project root
    pub path: PathBuf,
}

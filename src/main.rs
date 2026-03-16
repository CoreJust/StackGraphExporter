mod artifacts;
mod cfl_builder;
mod cfl_query;
mod cli;
mod core;
mod error;
mod io;
mod loading;
mod sg_builder;
mod sg_query;

use error::Result;

fn main() -> Result<()> {
    cli::run()
}

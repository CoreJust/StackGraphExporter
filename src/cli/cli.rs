use crate::{
    cli::{
        args::{Cli, Commands, OpenArgs},
        command_processor::{Command, CommandProcessor},
        engine::Engine,
    },
    error::{Error, Result},
};
use clap::Parser;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Open(args) => run_open(args),
    }
}

fn run_open(args: OpenArgs) -> Result<()> {
    let symbol = args.symbol.clone();
    let source = args.source.clone();
    let pick_queries = args.pick_queries.clone();
    let query_all_paths = args.query_all_paths;
    let immediates_count: u32 = [
        symbol.is_some(),
        source.is_some(),
        pick_queries.is_some(),
        query_all_paths,
    ]
    .iter()
    .map(|&some| if some { 1 } else { 0 })
    .sum();
    if immediates_count > 1 {
        return Err(Error::InvalidArgument("Can only have one immediate query argument at a time, but {immediates_count} were given".into()));
    }

    let path = args.path.clone();

    let engine = Engine::new(args);
    let mut processor = CommandProcessor::new(engine);

    let mut commands = vec![Command::Open { path }];
    if let Some(sym) = &symbol {
        commands.push(Command::Create { artifact: None });
        commands.push(Command::QuerySymbol {
            symbol: sym.clone(),
        });
    } else if let Some(src) = &source {
        commands.push(Command::Create { artifact: None });

        let parts: Vec<&str> = src.split(':').collect();
        if parts.len() != 3 {
            return Err(Error::InvalidArgument(
                "Invalid source format. Use path:line:col".into(),
            ));
        }
        let file = parts[0].to_string();
        let line = parts[1].parse::<usize>()?;
        let col = parts[2].parse::<usize>()?;

        let node_idx = processor.engine.find_node_at_source(&file, line, col)?;
        commands.push(Command::QueryNode { node: node_idx });
    } else if let Some(count) = pick_queries {
        commands.push(Command::Create { artifact: None });
        commands.push(Command::PickQueries { count });
    } else if query_all_paths {
        commands.push(Command::Create { artifact: None });
        commands.push(Command::QueryAll);
    }

    for cmd in commands {
        processor.process(cmd)?;
    }

    if immediates_count == 0 {
        crate::cli::interactive::run_interactive(processor)?;
    }

    Ok(())
}

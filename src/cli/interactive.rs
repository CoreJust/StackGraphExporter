use crate::{
    cli::{
        command_processor::{Command, CommandProcessor},
        engine::ArtifactType,
    },
    error::{Error, Result},
    io::read_line,
};

pub fn run_interactive(mut processor: CommandProcessor) -> Result<()> {
    loop {
        let line = read_line("> ")?;
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "exit" | "quit" | "halt" => {
                processor.process(Command::Exit)?;
                break;
            }
            "help" | "h" => {
                processor.process(Command::Help)?;
            }
            "create" | "c" => {
                if parts.len() > 2 {
                    crate::error!("Usage: create <artifact>");
                    continue;
                }
                let artifact = if parts.len() == 1 {
                    None
                } else {
                    match parse_artifact(parts[1]) {
                        Some(artifact) => Some(artifact),
                        None => {
                            return Err(Error::InvalidArgument(format!(
                                "Unknown artifact '{}'",
                                parts[1]
                            )));
                        }
                    }
                };
                if let Err(e) = processor.process(Command::Create { artifact }) {
                    crate::error!("{}", e);
                }
            }
            "query" | "q" | "r" | "run" => {
                if parts.len() < 2 {
                    crate::error!("Usage: query <symbol>");
                    continue;
                }
                let symbol = parts[1].to_string();
                if let Err(e) = processor.process(Command::QuerySymbol { symbol }) {
                    crate::error!("{}", e);
                }
            }
            "enable" | "e" => {
                if parts.len() < 2 {
                    crate::error!("Usage: enable <feature>");
                    continue;
                }
                let feature = parts[1].to_string();
                if let Err(e) = processor.process(Command::Enable { feature }) {
                    crate::error!("{}", e);
                }
            }
            "disable" | "d" => {
                if parts.len() < 2 {
                    crate::error!("Usage: disable <feature>");
                    continue;
                }
                let feature = parts[1].to_string();
                if let Err(e) = processor.process(Command::Disable { feature }) {
                    crate::error!("{}", e);
                }
            }
            "output" | "o" => {
                if parts.len() < 2 {
                    crate::error!("Usage: output [artifact] <path>");
                    continue;
                }
                if parts.len() == 2 {
                    let path = std::path::PathBuf::from(parts[1]);
                    if let Err(e) = processor.process(Command::Output {
                        artifact: None,
                        path,
                    }) {
                        crate::error!("{}", e);
                    }
                } else {
                    let artifact_str = parts[1];
                    let path = std::path::PathBuf::from(parts[2]);
                    match parse_artifact(artifact_str) {
                        Some(artifact) => {
                            if let Err(e) = processor.process(Command::Output {
                                artifact: Some(artifact),
                                path,
                            }) {
                                crate::error!("{}", e);
                            }
                        }
                        None => {
                            crate::error!("Unknown artifact '{}'", artifact_str);
                        }
                    }
                }
            }
            "state" | "s" => {
                if let Err(e) = processor.process(Command::State) {
                    crate::error!("{}", e);
                }
            }
            _ => {
                crate::error!("Unknown command. Type 'help'.");
            }
        }
    }
    Ok(())
}

fn parse_artifact(s: &str) -> Option<ArtifactType> {
    match s {
        "cfg" => Some(ArtifactType::Cfg),
        "csv" => Some(ArtifactType::Csv),
        "dot" => Some(ArtifactType::Dot),
        "dot-ucfs" => Some(ArtifactType::DotUcfs),
        "kt" => Some(ArtifactType::Kt),
        "json" => Some(ArtifactType::Json),
        _ => None,
    }
}

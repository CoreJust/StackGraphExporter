#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactType {
    Cfg,
    Csv,
    Dot,
    DotUcfs,
    Kt,
    Json,
    G,
    GCfg,
    Cnf,
    CnfCfg,
}

impl ArtifactType {
    pub fn parse(s: &str) -> Option<ArtifactType> {
        match s {
            "cfg" => Some(ArtifactType::Cfg),
            "csv" => Some(ArtifactType::Csv),
            "dot" => Some(ArtifactType::Dot),
            "dot_ucfs" | "dot-ucfs" => Some(ArtifactType::DotUcfs),
            "kt" => Some(ArtifactType::Kt),
            "json" => Some(ArtifactType::Json),
            "g" => Some(ArtifactType::G),
            "g_cfg" | "g-cfg" => Some(ArtifactType::GCfg),
            "cnf" => Some(ArtifactType::Cnf),
            "cnf_cfg" | "cnf-cfg" => Some(ArtifactType::CnfCfg),
            _ => None,
        }
    }

    pub fn default_file_name(&self) -> &'static str {
        match self {
            ArtifactType::Cfg => "cfl_grammar.cfg",
            ArtifactType::Csv => "cfl.csv",
            ArtifactType::Dot => "stackgraph.dot",
            ArtifactType::DotUcfs => "cfl_ucfs.dot",
            ArtifactType::Kt => "UCFSGrammar.kt",
            ArtifactType::Json => "stackgraph.json",
            ArtifactType::G => "cfl.g",
            ArtifactType::GCfg => "cfl.cfg.g",
            ArtifactType::Cnf => "cfl_grammar.cnf",
            ArtifactType::CnfCfg => "cfl_grammar.cfg.cnf",
        }
    }
}

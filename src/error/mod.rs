use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error for `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Stack graph building error: {0}")]
    StackGraphBuild(#[from] tree_sitter_stack_graphs::BuildError),

    #[error("Stack graph language error: {0}")]
    StackGraphLanguage(#[from] tree_sitter_stack_graphs::LanguageError),

    #[error("Path extraction error: {0}")]
    PathExtraction(String),

    #[error("CFL conversion error: {0}")]
    CflConversion(String),

    #[error("Query execution error: {0}")]
    Query(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn io_error(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Error::Io {
            path: path.into(),
            source,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Internal(value.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::Internal(value.to_string())
    }
}

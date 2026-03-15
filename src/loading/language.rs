use crate::error::{Error, Result};
use tree_sitter_stack_graphs::StackGraphLanguage;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Python,
    Java,
}

impl Language {
    pub fn from_str(name: &str) -> Result<Self> {
        match name {
            "py" | "python" => Ok(Language::Python),
            "java" => Ok(Language::Java),
            _ => Err(Error::InvalidArgument(format!(
                "Unsupported language '{}'. Supported: py, python, java",
                name,
            ))),
        }
    }

    pub fn file_extensions(&self) -> &'static [&'static str] {
        match self {
            Language::Python => &["py"],
            Language::Java => &["java"],
        }
    }

    pub fn build_stack_graph_language(&self) -> Result<StackGraphLanguage> {
        match self {
            Language::Python => {
                let tsg_src = tree_sitter_stack_graphs_python::STACK_GRAPHS_TSG_SOURCE;
                let tsg_lang = tree_sitter_python::LANGUAGE.into();
                Ok(StackGraphLanguage::from_str(tsg_lang, tsg_src)?)
            }
            Language::Java => {
                let tsg_src = tree_sitter_stack_graphs_java::STACK_GRAPHS_TSG_SOURCE;
                let tsg_lang = tree_sitter_java::LANGUAGE.into();
                Ok(StackGraphLanguage::from_str(tsg_lang, tsg_src)?)
            }
        }
    }
}

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::{
    error::{Error, Result},
    loading::Language,
};

pub fn discover_files(project_dir: &Path, language: &Language) -> Result<Vec<PathBuf>> {
    if !project_dir.is_dir() {
        return Err(Error::InvalidArgument(format!(
            "Path is not a directory: {}",
            project_dir.display(),
        )));
    }

    let extensions = language.file_extensions();
    Ok(WalkDir::new(project_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| extensions.contains(&ext))
                .unwrap_or(false)
        })
        .collect())
}

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::error::Result;

pub fn discover_files(project_dir: &Path, extensions: &[&str]) -> Result<Vec<PathBuf>> {
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

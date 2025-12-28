use anyhow::Result;
use stack_graphs::graph::StackGraph;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tree_sitter_stack_graphs::{NoCancellation, StackGraphLanguage, Variables};
use walkdir::WalkDir;

use tree_sitter_java;
use tree_sitter_python;
use tree_sitter_stack_graphs_java as j_tsg;
use tree_sitter_stack_graphs_python as py_tsg;

fn project_walkdir(project_dir: &String) -> Result<WalkDir> {
    let project_root = PathBuf::from(&project_dir);
    if !project_root.is_dir() {
        anyhow::bail!(
            "Provided path is not a directory: {}\n\
            If you intended to analyze a single file, run: cargo run -- <path-to-file>\n\
            Otherwise, pass the path to the project directory.",
            project_dir
        );
    }
    Ok(WalkDir::new(&project_root))
}

fn walk_project<F: FnMut(String, String) -> Result<()>>(
    project_dir: &String,
    valid_exts: &[&str],
    mut func: F,
) -> Result<()> {
    for entry in project_walkdir(project_dir)?
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if !valid_exts.contains(&ext) {
                continue;
            }
        } else {
            continue;
        }

        let mut src = String::new();
        File::open(path)?.read_to_string(&mut src)?;

        let file_path_str = path.to_string_lossy().to_string();
        func(src, file_path_str)?;
    }
    Ok(())
}

fn get_language(language: &String) -> Result<StackGraphLanguage> {
    if language == "py" {
        let tsg_src = py_tsg::STACK_GRAPHS_TSG_SOURCE;
        let tsg_lang = tree_sitter_python::LANGUAGE.into();
        Ok(StackGraphLanguage::from_str(tsg_lang, tsg_src)?)
    } else {
        let tsg_src = j_tsg::STACK_GRAPHS_TSG_SOURCE;
        let tsg_lang = tree_sitter_java::LANGUAGE.into();
        Ok(StackGraphLanguage::from_str(tsg_lang, tsg_src)?)
    }
}

pub fn load_graph(project_dir: &String, language: &String) -> Result<StackGraph> {
    let sg_language = get_language(language)?;

    let mut stack_graph = StackGraph::new();
    let variables = Variables::new();
    let cancel_flag = NoCancellation;

    let valid_exts = [language.as_str()];

    walk_project(project_dir, &valid_exts, |src, file_path_str| {
        let file_handle = stack_graph.get_or_create_file(file_path_str.as_str());
        sg_language.build_stack_graph_into(
            &mut stack_graph,
            file_handle,
            src.as_str(),
            &variables,
            &cancel_flag,
        )?;
        Ok(())
    })?;
    Ok(stack_graph)
}

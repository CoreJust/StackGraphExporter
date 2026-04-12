use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::error::{Error, Result};
use crate::loading::{discover_files, Language};
use crate::unsupported_features_cleaner::java_parser::JavaParser;
use crate::unsupported_features_cleaner::progress_event::ProgressEvent;
use crate::unsupported_features_cleaner::token::{Token, TokenType};

pub fn clean_unsupported_features<F>(
    project_dir: &Path,
    language: &Language,
    mut progress: F,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let marker = project_dir.join(".unsupported_features_cleaned");
    if marker.exists() {
        progress(ProgressEvent::DoneCached)?;
        return Ok(());
    }

    if let Language::Python = language {
        return Err(Error::Internal("Unsupported language: Python".into()));
    }

    let start = Instant::now();
    let file_paths = discover_files(project_dir, &language)?;

    progress(ProgressEvent::FilesFound {
        count: file_paths.len(),
        elapsed: start.elapsed(),
    })?;

    let total = file_paths.len();
    let mut removed_imports = 0usize;
    let mut removed_static_scopes = 0usize;
    let mut fixed_c_style_arrays = 0usize;
    let mut removed_comments = 0usize;

    for (i, path) in file_paths.iter().enumerate() {
        progress(ProgressEvent::FileStarted {
            path: path.clone(),
            processed: i,
            total,
            elapsed: start.elapsed(),
        })?;

        process_file(
            path,
            &mut removed_imports,
            &mut removed_static_scopes,
            &mut fixed_c_style_arrays,
            &mut removed_comments,
        )?;
    }

    fs::write(marker, b"cleaned")?;

    progress(ProgressEvent::Done {
        removed_imports,
        removed_static_scopes,
        fixed_c_style_arrays,
        removed_comments,
        elapsed: start.elapsed(),
    })?;

    Ok(())
}

fn process_file(
    path: &Path,
    removed_imports: &mut usize,
    removed_static_scopes: &mut usize,
    fixed_c_style_arrays: &mut usize,
    removed_comments: &mut usize,
) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let content = clean_imports(&content, removed_imports);
    let content = remove_static_scopes(&content, removed_static_scopes);
    let content = fix_c_style_arrays(&content, fixed_c_style_arrays);
    let content = remove_comments(&content, removed_comments);
    fs::write(path, content)?;
    Ok(())
}

fn clean_imports(input: &str, removed_imports: &mut usize) -> String {
    input
        .lines()
        .filter(|line| {
            let l = line.trim_start();
            if l.starts_with("import ") && (l.contains("static") || l.contains(".*")) {
                *removed_imports += 1;
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn remove_static_scopes(src: &str, removed_static_scopes: &mut usize) -> String {
    let mut parser = JavaParser::new(src);
    let mut tokens = Vec::new();
    while let Some(tok) = parser.next() {
        tokens.push(tok);
    }

    let mut out = String::with_capacity(src.len());
    let mut cursor = 0usize;
    let mut i = 0usize;

    while i < tokens.len() {
        let tok = &tokens[i];

        if tok.text == "static" {
            let mut j = i + 1;
            while j < tokens.len() && matches!(tokens[j].ty, TokenType::Comment { .. }) {
                j += 1;
            }

            if j < tokens.len() && tokens[j].text == "{" {
                let brace_pos = tokens[j].at;

                if let Some(end) = parser.find_matching_brace_end(brace_pos) {
                    out.push_str(&src[cursor..tok.at]);
                    cursor = end;
                    *removed_static_scopes += 1;

                    while i < tokens.len() && tokens[i].at < end {
                        i += 1;
                    }
                    continue;
                }
            }
        }

        let token_end = tok.at + tok.text.len();
        if cursor < token_end {
            out.push_str(&src[cursor..token_end]);
            cursor = token_end;
        }

        i += 1;
    }

    out.push_str(&src[cursor..]);
    out
}

fn remove_comments(src: &str, removed: &mut usize) -> String {
    let mut out = String::with_capacity(src.len());
    let mut cursor = 0;
    let mut parser = JavaParser::new(src);
    while let Some(token) = parser.next() {
        match token.ty {
            TokenType::Comment { .. } => {
                out.push_str(&src[cursor..token.at]);
                *removed += 1;
                cursor = token.at + token.text.len();
            }
            _ => {}
        }
    }
    out.push_str(&src[cursor..]);
    out
}

fn is_comment(tok: &Token<'_>) -> bool {
    matches!(tok.ty, TokenType::Comment { .. })
}

fn next_sig_idx<'a>(tokens: &[Token<'a>], mut idx: usize) -> Option<usize> {
    while idx < tokens.len() {
        if !is_comment(&tokens[idx]) {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

fn prev_sig_idx<'a>(tokens: &[Token<'a>], mut idx: usize) -> Option<usize> {
    while idx > 0 {
        idx -= 1;
        if !is_comment(&tokens[idx]) {
            return Some(idx);
        }
    }
    None
}

fn is_blocking_prev(text: &str) -> bool {
    matches!(
        text,
        "(" | "["
            | "{"
            | "="
            | ":"
            | "?"
            | "."
            | "new"
            | "return"
            | "throw"
            | "case"
            | "if"
            | "for"
            | "while"
            | "switch"
            | "catch"
            | "do"
            | "else"
    )
}

fn is_declaration_delimiter(text: &str) -> bool {
    matches!(text, "," | ";" | ")" | "=")
}

pub fn fix_c_style_arrays(src: &str, removed: &mut usize) -> String {
    let mut parser = JavaParser::new(src);
    let mut tokens = Vec::new();
    while let Some(tok) = parser.next() {
        tokens.push(tok);
    }

    if tokens.is_empty() {
        return src.to_owned();
    }

    let mut remove = vec![false; tokens.len()];

    let mut chunk_start_stack: Vec<usize> = Vec::new();
    let mut chunk_start = 0usize;

    let mut i = 0usize;
    while i < tokens.len() {
        let tok = &tokens[i];

        match tok.text {
            "(" | "{" => {
                chunk_start_stack.push(chunk_start);
                chunk_start = next_sig_idx(&tokens, i + 1).unwrap_or(tokens.len());
                i += 1;
                continue;
            }
            ")" | "}" => {
                chunk_start = chunk_start_stack.pop().unwrap_or(0);
                i += 1;
                continue;
            }
            ";" => {
                chunk_start = next_sig_idx(&tokens, i + 1).unwrap_or(tokens.len());
                i += 1;
                continue;
            }
            _ => {}
        }

        let is_candidate_ident = matches!(tok.ty, TokenType::Id { .. }) && !tok.is_keyword();
        if !is_candidate_ident {
            i += 1;
            continue;
        }

        let Some(prev) = prev_sig_idx(&tokens, i) else {
            i += 1;
            continue;
        };
        if prev < chunk_start {
            i += 1;
            continue;
        }
        if is_blocking_prev(tokens[prev].text) {
            i += 1;
            continue;
        }

        // Count consecutive `[]` pairs immediately after the identifier.
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        let mut scan_from = i;

        loop {
            let Some(lb) = next_sig_idx(&tokens, scan_from + 1) else {
                break;
            };
            if tokens[lb].text != "[" {
                break;
            }

            let Some(rb) = next_sig_idx(&tokens, lb + 1) else {
                break;
            };
            if tokens[rb].text != "]" {
                break;
            }

            pairs.push((lb, rb));
            scan_from = rb;
        }

        if pairs.is_empty() {
            i += 1;
            continue;
        }

        let after = next_sig_idx(&tokens, scan_from + 1).unwrap_or(tokens.len());
        let ok_after = after < tokens.len() && is_declaration_delimiter(tokens[after].text);

        if ok_after {
            for (lb, rb) in &pairs {
                remove[*lb] = true;
                remove[*rb] = true;
            }
            *removed += pairs.len() * 2;
        }

        i += 1;
    }

    let mut out = String::with_capacity(src.len());
    let mut last = 0usize;

    for (idx, tok) in tokens.iter().enumerate() {
        if remove[idx] {
            out.push_str(&src[last..tok.at]);
            last = tok.at + tok.text.len();
        }
    }

    out.push_str(&src[last..]);
    out
}

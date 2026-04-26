use crate::{
    artifacts::{grammar_kt_pieces::*, progress_event::ProgressEvent},
    core::CFLGraph,
    error::Result,
    io::ElapsedAndCount,
};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

const WRITE_ONCE_IN_N: usize = 64;

pub trait ToKTGrammar {
    fn to_kotlin_lines<F>(&self, class_name: &str, progress: &mut F) -> Result<Vec<String>>
    where
        F: FnMut(ProgressEvent) -> Result<()>;

    fn write_to_kotlin_file<F>(
        &self,
        out_path: &PathBuf,
        class_name: &str,
        mut progress: F,
    ) -> Result<()>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let mut file = File::create(out_path)?;
        let start = Instant::now();
        let kt = self.to_kotlin_lines(class_name, &mut progress)?;
        let total_lines = kt.len();
        for (i, line) in kt.into_iter().enumerate() {
            writeln!(file, "{line}")?;
            if i % WRITE_ONCE_IN_N == 0 {
                progress(ProgressEvent::WritingLines {
                    elapsed_and_count: ElapsedAndCount {
                        current: i,
                        total: total_lines,
                        elapsed: start.elapsed(),
                    },
                    artifact_name: "Kotlin Grammar",
                })?;
            }
        }
        progress(ProgressEvent::ArtifactStored {
            elapsed_and_count: ElapsedAndCount {
                current: total_lines,
                total: total_lines,
                elapsed: start.elapsed(),
            },
            artifact_name: "Kotlin Grammar",
        })
    }
}

impl ToKTGrammar for CFLGraph {
    fn to_kotlin_lines<F>(&self, class_name: &str, progress: &mut F) -> Result<Vec<String>>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        KotlinGrammarGenerator::new(self, class_name, progress).generate()
    }
}

struct KotlinGrammarGenerator<'a, F>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    graph: &'a CFLGraph,
    class_name: &'a str,
    progress: &'a mut F,
    start: Instant,
}

impl<'a, F> KotlinGrammarGenerator<'a, F>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    fn new(graph: &'a CFLGraph, class_name: &'a str, progress: &'a mut F) -> Self {
        Self {
            graph,
            class_name,
            progress,
            start: Instant::now(),
        }
    }

    fn generate(&mut self) -> Result<Vec<String>> {
        let mut kt_lines = vec![KT_GRAMMAR_HEADER.to_string()];
        self.append_class_declaration(&mut kt_lines);
        self.append_non_terminal_declarations(&mut kt_lines)?;
        self.append_init_block(&mut kt_lines, self.graph.sg_unique_symbols_count)?;

        kt_lines.push("}".to_string());
        Ok(kt_lines)
    }

    fn append_class_declaration(&self, kt_lines: &mut Vec<String>) {
        kt_lines.push(format!("class {} : Grammar() {{\n", self.class_name));
    }

    fn append_non_terminal_declarations(&mut self, kt_lines: &mut Vec<String>) -> Result<()> {
        kt_lines.push(format!(
            "\tval S by Nt().asStart() // <placeholder nt=\"S\"/>"
        ));
        Ok(())
    }

    fn append_init_block(
        &mut self,
        kt_lines: &mut Vec<String>,
        sg_symbols_count: usize,
    ) -> Result<()> {
        kt_lines.push("\tinit {".to_string());
        (self.progress)(ProgressEvent::GeneratingArtifact {
            elapsed: self.start.elapsed(),
            progress: None,
            message: "Appending init block".into(),
        })?;

        kt_lines.push(kt_grammar_productions_map_build(sg_symbols_count));
        kt_lines.push("\t}".to_string());
        Ok(())
    }
}

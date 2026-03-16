use std::path::PathBuf;

use crate::core::CFLGraph;
use crate::error::Result;

pub trait ToCSV {
    fn to_csv_table(self: &Self) -> Vec<(String, Vec<String>)>; // Vec of columns

    fn write_to_csv_file(self: &Self, out_path: &PathBuf, print_titles: bool) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut out_file = File::create(&out_path)?;
        let csv_table = self.to_csv_table();
        assert!(csv_table.iter().all(|c| c.1.len() == csv_table[0].1.len()));

        if print_titles {
            writeln!(
                out_file,
                "{}",
                csv_table
                    .iter()
                    .map(|c| c.0.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }
        for i in 0..csv_table[0].1.len() {
            writeln!(
                out_file,
                "{}",
                csv_table
                    .iter()
                    .map(|c| c.1[i].as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        Ok(())
    }
}

impl ToCSV for CFLGraph {
    fn to_csv_table(self: &Self) -> Vec<(String, Vec<String>)> {
        let mut column_from = Vec::<String>::with_capacity(self.edges.len());
        let mut column_to = Vec::<String>::with_capacity(self.edges.len());
        let mut column_label = Vec::<String>::with_capacity(self.edges.len());
        for edge in &self.edges {
            column_from.push(format!("{}", edge.from));
            column_to.push(format!("{}", edge.to));
            column_label.push(format!(
                "{}",
                edge.symbol
                    .and_then(|s| Some(self.symbols[s].as_str()))
                    .unwrap_or("eps")
            ));
        }
        vec![
            ("from".to_owned(), column_from),
            ("to".to_owned(), column_to),
            ("label".to_owned(), column_label),
        ]
    }
}

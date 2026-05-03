// Based on https://github.com/FormalLanguageConstrainedPathQuerying/CFPQ_PyAlgo/blob/murav/optimize-matrix/docs/cli.md#grammar-format
use std::path::PathBuf;

use crate::error::Result;

pub fn write_to_cnf_cfg_file(out_path: &PathBuf, inverse_grammar: bool) -> Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut out_file = File::create(&out_path)?;

    if inverse_grammar {
        writeln!(
            out_file,
            "S
S eps S
S Q S
S V S
S#pp_i pp_i S
S#vpp_i vpp_i S
V S#vpp_i vpsh_i
Q S#pp_i psh_i

Count:
Q"
        )?;
    } else {
        writeln!(
            out_file,
            "S
S eps S
S Q S
S V S
S#psh_i psh_i S
S#vpsh_i vpsh_i S
V S#vpsh_i vpp_i
Q S#psh_i pp_i

Count:
Q"
        )?;
    }
    Ok(())
}

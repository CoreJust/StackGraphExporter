pub const KT_GRAMMAR_HEADER: &'static str = "\
package sg_bench

import org.ucfs.grammar.combinator.Grammar
import org.ucfs.grammar.combinator.regexp.*
import org.ucfs.rsm.symbol.Term
";

pub fn kt_grammar_productions_map_build(
    sg_symbols_count: usize,
    for_query_generation: bool,
) -> String {
    if for_query_generation {
        format!(
            "\t\tS /= (
\t\t\t(0..{sg_symbols_count})
\t\t\t\t.map {{ i -> Term(\"psh$i\") * S * Term(\"pp$i\") }}
\t\t\t\t.fold<Regexp, Regexp>(Empty) {{ acc, p -> acc or p }}
\t\t\tor Term(\"\")
\t\t).many"
        )
    } else {
        format!(
            "\t\tQ /= (0..{sg_symbols_count})
    \t\t\t.map {{ i -> Term(\"psh$i\") * S * Term(\"pp$i\") }}
    \t\t\t.fold<Regexp, Regexp>(Empty) {{ acc, p -> acc or p }}"
        )
    }
}

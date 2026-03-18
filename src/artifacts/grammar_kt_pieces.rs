pub const KT_GRAMMAR_HEADER: &'static str = "\
package sg_bench

import org.ucfs.grammar.combinator.Grammar
import org.ucfs.grammar.combinator.regexp.*
import org.ucfs.rsm.symbol.Term
";

pub const KT_GRAMMAR_PARSE_PRODUCTION_DATA: &'static str = "
\tprivate fun parseProductionData(data: String): Map<String, List<List<String>>> {
\t\tval map = mutableMapOf<String, MutableList<List<String>>>()
\t\tval lines = data.lines()
\t\tvar i = 0
\t\twhile (i < lines.size) {
\t\t\tval line = lines[i].trim()
\t\t\tif (line.isEmpty()) { i++; continue }
\t\t\tval nt = line
\t\t\ti++
\t\t\tval alternatives = mutableListOf<List<String>>()
\t\t\twhile (i < lines.size && lines[i].trim().isNotEmpty()) {
\t\t\t\tval altLine = lines[i].trim()
\t\t\t\tval tokens = mutableListOf<String>()
\t\t\t\tvar j = 0
\t\t\t\twhile (j < altLine.length) {
\t\t\t\t\tif (altLine[j] == ' ') { j++; continue }
\t\t\t\t\tif (altLine[j] == '\"') {
\t\t\t\t\t\tj++ // skip opening quote
\t\t\t\t\t\tval start = j
\t\t\t\t\t\twhile (j < altLine.length && altLine[j] != '\"') {
\t\t\t\t\t\t\tif (altLine[j] == '\\\\') j++ // skip escape
\t\t\t\t\t\t\tj++
\t\t\t\t\t\t}
\t\t\t\t\t\tval token = altLine.substring(start, j)
\t\t\t\t\t\ttokens.add(\"\\\"$token\\\"\") // keep quotes
\t\t\t\t\t\tj++ // skip closing quote
\t\t\t\t\t} else {
\t\t\t\t\t\tval start = j
\t\t\t\t\t\twhile (j < altLine.length && altLine[j] != ' ') j++
\t\t\t\t\t\ttokens.add(altLine.substring(start, j))
\t\t\t\t\t}
\t\t\t\t}
\t\t\t\talternatives.add(tokens)
\t\t\t\ti++
\t\t\t}
\t\t\tmap[nt] = alternatives
\t\t}
\t\treturn map
\t}

";

pub fn kt_grammar_productions_map_build(nt_names: Vec<String>) -> String {
    let nt_list = nt_names.join("\", \"");
    let branches = nt_names
        .into_iter()
        .map(|name| format!("\t\t\t\t\"{}\" -> {} /= combined", name, name))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "
\t\tval productionsMap = parseProductionData(productionData)
\t\tval ntNames = listOf(\"{nt_list}\")
\t\tfor (ntName in ntNames) {{
\t\t\tval alternatives = productionsMap[ntName] ?: emptyList()
\t\t\tval altList = alternatives.map {{ altTokens ->
\t\t\t\taltTokens.map {{ token ->
\t\t\t\t\twhen {{
\t\t\t\t\t\ttoken == \"Epsilon\" -> Epsilon
\t\t\t\t\t\ttoken.startsWith('\"') -> {{
\t\t\t\t\t\t\tval unquoted = token.substring(1, token.length - 1)
\t\t\t\t\t\t\t\t\t.replace(\"\\\\\\\"\", \"\\\"\")
\t\t\t\t\t\t\t\t\t.replace(\"\\\\\\\\\", \"\\\\\")
\t\t\t\t\t\t\tTerm(unquoted)
\t\t\t\t\t\t}}
\t\t\t\t\t\telse -> getNt(token)
\t\t\t\t\t}}
\t\t\t\t}}.reduce {{ acc, sym -> acc * sym }}
\t\t\t}}.toMutableList()

\t\t\taltList.add(0, Term(\"\") * S)
\t\t\taltList.add(1, S * Term(\"\"))

\t\t\tval combined = altList.reduce {{ acc, prod -> acc or prod }}
\t\t\twhen (ntName) {{
\t\t\t\t{branches}
\t\t\t\telse -> error(\"Unexpected non-terminal $ntName\")
\t\t\t}}
\t\t}}
"
    )
}

pub fn kt_grammar_get_nt(branches: Vec<&String>) -> String {
    format!(
        "
\tprivate fun getNt(name: String): Regexp = when (name) {{
\t\t{}
\t\telse -> error(\"Unknown non-terminal $name\")
\t}}
",
        branches
            .into_iter()
            .map(|name| format!("\t\t\"{}\" -> {}", name, name))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

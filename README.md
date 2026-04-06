# StackGraph Exporter

A CLI utility to generate stack graphs from source code folders. It allows to write the generated stack graph into DOT or JSON files, or to convert them into CFL-r task - where you can generate corresponding DOT/CSV graphs and CFG/Kotlin grammar. CFG grammar and CSV graphs are generated in the format supported by [KotGLL](https://github.com/vadyushkins/kotgll), Kotlin grammar and DOT graphs are generated in the format supported by [UCFS](https://github.com/FormalLanguageConstrainedPathQuerying/UCFS).

Suported languages: Java (complete support) and Python (partial support, only for Stack Graph generation).

## Implementation notes

For Java, some features of the language are not supported (because they are not supported by the crate [tree-sitter-stack-graphs-java](https://crates.io/crates/tree-sitter-stack-graphs-java)):

1. Static imports
2. Static scopes (i.e. `static { /* some code */ }` in the class scope)
3. Imports with asterisks (`import some.package.*`)
4. C-style arrays
5. Some comments, e.g.
```Java
public int f(int v,
// int deprecated,
    int q);
```

## Sample workflow

```
> cargo run -- open ../JsonPath/json-path/src/main/java/com/jayway/jsonpath
[00:00:05] [########################################] 100% [5834ms] Stack graph built successfully
success: Loaded project at ../JsonPath/json-path/src/main/java/com/jayway/jsonpath

> e ucfs
info: Enabled ucfs

> q evaluate
[00:00:01] [########################################] 100% [1632ms] SGGraph built successfully
[00:00:04] [########################################] 100% [3962ms] Database built successfully
[00:00:00] [########################################] 100% [339ms] Indexed nodes at partial path start
[00:00:00] [########################################] 100% [18ms] Found 103 references and 33 definitions for symbol 'evaluate'
info: Found 33 references:
  [0] node 17125 at ../JsonPath/json-path/src/main/java/com/jayway/jsonpath\internal\filter\EvaluatorFactory.java:60:58
  [1] node 17265 at ../JsonPath/json-path/src/main/java/com/jayway/jsonpath\internal\filter\EvaluatorFactory.java:67:60
  ...
  [32] node 114196 at ../JsonPath/json-path/src/main/java/com/jayway/jsonpath\JsonPath.java:348:51
Enter index to resolve (or 'a' for all, empty to cancel):

> 13
[00:00:00] [########################################] 100% [2ms] Paths stitched successfully
info: [0] Node 71622 resolves to 2 definitions:
  - ../JsonPath/json-path/src/main/java/com/jayway/jsonpath\internal\path\CompiledPath.java:105:29 local_id 1373
  - ../JsonPath/json-path/src/main/java/com/jayway/jsonpath\internal\path\CompiledPath.java:90:29 local_id 1071
[00:00:00] [########################################] 100% [189ms] CFL graph built successfully
[00:00:00] [########################################] 100% [23ms] UCFS query grammar generated
info: UCFS query DOT generated at .\query.cfl_ucfs.dot
info: UCFS query grammar generated at .\UCFSGrammar.kt

> exit
```

## CLI

To run the CLI, you can enter `<evecutable> open <path-to-source-files-root>`, which loads the code into a stack graph and starts interactive mode where you can run commands to configure the project, generate artifacts, run queries.

`<executable>` here can be either the actual executable or `cargo run --`, which allows to build and run with the following arguments.

Run `<executable> open --help` to see all available flags.

Available commands for open:

1. Language choice (the language of the code to be loaded into stack graph):
```
  -j, --java
  -p, --python
```

By default Java is assumed.

2. Backend choice:
```
      --kotgll
      --ucfs
```

By default both are disabled and all queries run solely against stack graphs.

When KotGLL is enabled, the corresponding artifacts are produced and KotGLL jar is invoked directly, then it's output is parsed and printed.

For UCFS, only the corresponding artifacts (Kotling grammar and DOT graph) are produced which you need to integrate with UCFS yourself.

3. KotGLL-related flags:
```
      --sppf
      --kotgll-path <KOTGLL_PATH>
```

SPPF enables output in the format of SPPF. KotGLL path is required to be provided when using KotGLL (it must be an executable JAR).

4. Artifact generation flags:
```
      --cfg
      --csv
      --stack-graph-dot or --sg-dot
      --dot-ucfs
      --kt
      --stack-graph-json or --sg-json
```

Enable generation of corresponding artifacts (those not marked with `stack-graph` or `sg` are for CFL).

Note that the artifacts are not generated automatically, you need to either run `create` without arguments later in interactive mode or make an immediate query (see Immediate query below).

5. Artifact output path flags:
```
  -o, --output <OUTPUT>
      --output-cfg <OUTPUT_CFG>
      --output-csv <OUTPUT_CSV>
      --output-stack-graph-dot or --output-sg-dot <OUTPUT_STACK_GRAPH_DOT>
      --output-dot-ucfs <OUTPUT_DOT_UCFS>
      --output-kt <OUTPUT_KT>
      --output-stack-graph-json or --output-sg-json <OUTPUT_STACK_GRAPH_JSON>
```

`-o, --output` sets directory for all the artifacts, others override paths for specific artifacts. By default, directory for all the artifacts is set to `./`.

6. Immediate query flags:
```
  -s, --symbol <SYMBOL>
      --source <SOURCE>
```

Immediately generates all the requested artifacts.

For `--source` you have to specify full path to the symbol (`<path-to-file>:<line>:<column>`) and then the query is immediately executed, then app exits.

For `--symbol` you have to specify symbol name and then you enter the query mode (see Query mode below), after which the app exits.

7. Other flags:
```
      --verify
```

Enables verification. When verification is enabled and the query is done with KotGLL, the results are parsed and compared to the results produced by stack graphs. **For UCFS verification is not implemented yet.**

```
      --all-symbols
```

By default, in the query mode you you only see nodes which are at the beginning of at least one partial path. You can disable this behaviour with this flag and see all the nodes for the symbol you requested.

**Note: even in medium-sized projects one symbol might have hundreds or thousands of nodes. Filtering them by having at least one partial path can reduce the number by several times. Emperically it was verified that nodes without partial paths are resolbed to nothing. But it must be further investigated.**

```
      --simplify-cfl
```

Currently produced CFL graphs have a lot of epsilon edges. Some might be easily pruned, which is enabled with this flag.

```
  -v, --verbose
  -h, --help
```

Self-explanatory. The former enables verbose output, the latter prints help information.

## Interactive mode

Available commands:

```
create, c [<artifact>]
query, q, run, r <symbol>
enable, e <feature>
disable, d <feature>
output, o [<artifact>] <path>
state, s
help, h
quit, exit, halt
```

<TODO>

### Query mode

Triggered by either using `--symbol` argument or running a query in interactive mode.

First, all the nodes that correspond to the symbol are found. Then, they are filtered by having at least one partial path that begins in a node (or not filtered if all-symbols feature is enabled, see `--all-symbols` above).

Those nodes are shown to the user and the user is given the choice: either query for all the nodes at once (enter `a`) or query for one specific node (enter that node's index in the list shown).

Then the query is run depending on enabled backends and features.

## Examples

<TODO>

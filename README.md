# StackGraph Exporter

A CLI utility to generate stack graphs from source code folders. It allows to write the generated stack graph into DOT or JSON files, or to convert them into CFL-r task - where you can generate corresponding DOT/CSV graphs and CFG/Kotlin grammar. CFG grammar and CSV graphs are generated in the format supported by [KotGLL](https://github.com/vadyushkins/kotgll), Kotlin grammar and DOT graphs are generated in the format supported by [UCFS](https://github.com/FormalLanguageConstrainedPathQuerying/UCFS).

Suported languages: Java (complete support) and Python (partial support, only for Stack Graph generation).

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
      --output-stack-graph-dot <OUTPUT_STACK_GRAPH_DOT>
      --output-dot-ucfs <OUTPUT_DOT_UCFS>
      --output-kt <OUTPUT_KT>
      --output-stack-graph-json <OUTPUT_STACK_GRAPH_JSON>
```

6. Immediate query flags:
```
  -s, --symbol <SYMBOL>
      --source <SOURCE>
```

Other flags:
  -q, --query
      --verify
  -v, --verbose
      --all-symbols
      --simplify-cfl
  -h, --help

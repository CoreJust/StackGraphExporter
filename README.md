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

Other flags:
  -q, --query
      --verify
  -v, --verbose
      --all-symbols
      --simplify-cfl
      --sppf
      --kotgll-path <KOTGLL_PATH>
      --cfg
      --csv
      --dot
      --dot-ucfs
      --kt
      --stack-graph-json
  -o, --output <OUTPUT>
      --output-cfg <OUTPUT_CFG>
      --output-csv <OUTPUT_CSV>
      --output-dot <OUTPUT_DOT>
      --output-dot-ucfs <OUTPUT_DOT_UCFS>
      --output-kt <OUTPUT_KT>
      --output-stack-graph-json <OUTPUT_STACK_GRAPH_JSON>
  -s, --symbol <SYMBOL>
      --source <SOURCE>
  -h, --help

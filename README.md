# StackGraph Exporter

A CLI utility to generate stack graphs from source code folders. It allows to write the generated stack graph into DOT or JSON files, or to convert them into CFL-r task - where you can generate corresponding DOT/CSV graphs and CFG/Kotlin grammar. CFG grammar and CSV graphs are generated in the format supported by [KotGLL](https://github.com/vadyushkins/kotgll), Kotlin grammar and DOT graphs are generated in the format supported by [UCFS](https://github.com/FormalLanguageConstrainedPathQuerying/UCFS).

Suported languages: Java (complete support) and Python (partial support, only for Stack Graph generation).

## CLI

To run the CLI, you can enter `<evecutable> open <path-to-source-files-root>`.

`<executable>` here can be either the actual executable or `cargo run --`, whick allows to build and run with the following arguments.

Run `<executable> open --help` to see all available flags.

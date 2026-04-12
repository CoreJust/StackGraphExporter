# Time measurements

## Environment and condtions

There was made a number of duration measurements for name resolution queries using Stack Graphs and CFL-r (UCFS). For Stack Graphs, this CLI utility was used, for CFL-r the generated artifacts were used to create benchmarks using UCFS and JMH.

Environment:

- Acer laptop with Windows 10.
- Processor: Intel Core i5-10300H
- Memory: 16 GB of DDR4 SDRAM

UCFS measurements were done with JMH, see [UCFS_JMH repo](https://github.com/CoreJust/UCFS_JMH) (run_sg_bench.ps1).

To run it yourself you need to:

1. You will need jabba and install `openjdk@1.17.0` with it. Or if you are sure that you JVM environment suits UCFS, you can skip it and later comment out the line `Invoke-Command -Command jabba -ArgumentList @('use', 'openjdk@1.17.0')` in run_sg_bench.ps1.

2. Clone that repo.

3. Put your grammar file to sg_bench/src/jmh/kotlin/sg_bench/UCFSGrammar.kt.

4. Put you DOT graph file somewhere near the project root (for convenience only).

5. Change the JVM arguments in run_sg_bench.ps1 to match your hardware.

6. Run ./run_sg_bench.ps1 <path-to-DOT-file>.

Note that that all works on Windows only - for other systems you have to rewrite the run_sg_bench.ps1 to match the system or take the required commands from there and type them manually.

## Raw results

Note that you cannot simply compare the results for Stack Graphs and UCFS - for Stack Graphs a significant part of the job is done beforehand during Partial Paths Database building.

### Project: [libgdx](https://github.com/libgdx/libgdx)

- Path to the code: libgdx/gdx/src/com/badlogic/gdx (3.4 MB of code)
- Stack Graph built in 01:17; It has 2 448 395 vertices, 2 168 813 edges; 260 510 symbols
- Partial Paths Database built in 01:58
- CFL graph built in 00:03 (00:12 with simplification enabled)
- Generated UCFS grammar file size: 492.3 KB; There are 260 512 rules
- Generated UCFS graph file size: 122.6 MB; Graph has 3 557 998 vertices, 3 278 417 edges
- Generated simplified UCFS graph size: 108.3 MB; Graph has 2 867 087 vertices, 2 886 186 edges

1. getTransformedVertices:0 at ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:126:24

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Polygon.java:32:16
  - Stack Graphs: 13ms +- 1ms
  - UCFS: 244s +- 28s
  - UCFS Simplified: 214s +- 42s

2. getText:20 at ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\XmlReader.java:699:24

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\XmlReader.java:507:16
  - Stack Graphs: 7ms +- 1ms
  - UCFS: 40.6s +- 5.6s
  - UCFS Simplified: 27.5s +- 0.9s

3. getX:29 at ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:547:83

  - Resolved to 2 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:370:14
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:375:14
  - Stack Graphs: 8ms +- 1ms
  - UCFS: 40.6s +- 5.6s
  - UCFS Simplified: 207.5s +- 27.8s

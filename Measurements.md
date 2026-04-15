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

For automatic query picking you can use `--pick-queries` (as described in README) to get the sgeq file and the intermediate artifacts, then use the scripts from UCFS:

```
python ./run_sgeq.py <path to the sgeq file> <path to the output MD file> [<path to UCFS root>, default ./]
```

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


### Project: [JsonPath](https://github.com/json-path/JsonPath)

- Path to the code: JsonPath\json-path\src\main\java\com\jayway\jsonpath (0.4 MB of code)
- Stack Graph built in 8.6s; It has 178,914 vertices, 153,255 edges; 23,071 symbols
- Partial Paths Database built in 5.1s
- CFL graph built in 273ms (955ms with simplification enabled)
- Generated UCFS grammar file size: 57.2 KB; There are 23,073 rules
- Generated UCFS graph file size: 7.7 MB; Graph has 253,715 vertices, 228,057 edges
- Generated simplified UCFS graph size: 6.8 MB; Graph has 200,411 vertices, 200,424 edges

1. Configuration at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:183:30

  - Resolved to 20 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\GsonMappingProvider.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicateContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\ParseContextImpl.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JacksonMappingProvider.java:5:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\JsonContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonSmartMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\TapestryMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\Jackson3MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\Path.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\latebinding\PathLateBindingValue.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:11:13
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JakartaMappingProvider.java:22:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\CompiledPath.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonOrgMappingProvider.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNodes.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\PathRef.java:2:27
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 9.4s ± 1.4s
  - UCFS Simplified: 8.6s ± 1.1s

2. Configuration at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:84:15

  - Resolved to 20 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\JsonContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNodes.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\TapestryMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\PathRef.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonSmartMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonOrgMappingProvider.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JakartaMappingProvider.java:22:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\GsonMappingProvider.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicateContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\Jackson3MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JacksonMappingProvider.java:5:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\Path.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\CompiledPath.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\latebinding\PathLateBindingValue.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:11:13
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\ParseContextImpl.java:2:27
  - Stack Graphs: 5ms ± 1ms
  - UCFS: 11.6s ± 1.9s
  - UCFS Simplified: 8.8s ± 798ms

3. asOffsetDateTimeNode at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:105:77

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNode.java:109:30
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 3.5s ± 307ms
  - UCFS Simplified: 1.7s ± 72ms

4. charAt at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:116:16

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:35:16
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 4.3s ± 287ms
  - UCFS Simplified: 1.6s ± 262ms

5. options at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:163:68

  - Resolved to 5 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:118:32
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:138:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:30:30
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:131:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:138:56
  - Stack Graphs: 9ms ± 1ms
  - UCFS: 8.4s ± 1.1s
  - UCFS Simplified: 7.9s ± 592ms

6. asJsonNode at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:342:37

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNode.java:60:20
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 3.7s ± 1.2s
  - UCFS Simplified: 1.6s ± 189ms

7. valueNode at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:301:24

  - Resolved to 2 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:295:42
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:278:70
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 2.6s ± 132ms
  - UCFS Simplified: 934ms ± 155ms

8. asValueListNode at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:386:43

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNode.java:76:25
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 3.4s ± 372ms
  - UCFS Simplified: 1.8s ± 171ms

9. asValueListNode at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:346:44

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNode.java:76:25
  - Stack Graphs: 2ms ± 0ms
  - UCFS: <corrupted>
  - UCFS Simplified: 1.9s ± 306ms

10. right at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:131:65

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:127:58
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 1.9s ± 110ms
  - UCFS Simplified: 616ms ± 392ms

11. Collection at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:148:55

  - Resolved to 21 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\LogicalExpressionNode.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JsonOrgJsonProvider.java:14:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:14:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JettisonProvider.java:7:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\TapestryJsonProvider.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\GsonJsonProvider.java:18:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JakartaMappingProvider.java:13:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PathCompiler.java:12:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\Parameter.java:7:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\AbstractJsonProvider.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JakartaJsonProvider.java:27:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PathTokenFactory.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Filter.java:6:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JsonProvider.java:6:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ScanPathToken.java:7:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\PathRef.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\Jackson3JsonNodeJsonProvider.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Criteria.java:12:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JacksonJsonNodeJsonProvider.java:17:17
  - Stack Graphs: 4ms ± 1ms
  - UCFS: 7.7s ± 1.2s
  - UCFS Simplified: 7.7s ± 807ms

12. c at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:300:39

  - Resolved to 13 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:165:41
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:191:65
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:43:38
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:219:41
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:134:47
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:154:51
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:169:60
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:199:46
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:138:66
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:47:35
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:150:32
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:51:35
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:298:13
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 2.6s ± 391ms
  - UCFS Simplified: 2.2s ± 171ms

13. options at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:139:32

  - Resolved to 5 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:138:56
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:131:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:30:30
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:118:32
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:138:36
  - Stack Graphs: 8ms ± 0ms
  - UCFS: 8.9s ± 665ms
  - UCFS Simplified: 8.7s ± 1.4s

14. Configuration at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:103:18

  - Resolved to 20 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JacksonMappingProvider.java:5:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\CompiledPath.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\Path.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonOrgMappingProvider.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonSmartMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\PathRef.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\GsonMappingProvider.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\TapestryMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\JsonContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNodes.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:11:13
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicateContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\latebinding\PathLateBindingValue.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\ParseContextImpl.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\Jackson3MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JakartaMappingProvider.java:22:27
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 7.8s ± 770ms
  - UCFS Simplified: 7.7s ± 670ms

15. isPatternNode at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:255:22

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNode.java:16:19
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 2.5s ± 175ms
  - UCFS Simplified: 1.6s ± 157ms

16. input at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:299:16

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\EvaluatorFactory.java:296:19
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 2.1s ± 170ms
  - UCFS Simplified: 978ms ± 44ms


### Project: [Shattered Pixel Dungeon](https://github.com/00-Evan/shattered-pixel-dungeon)

*Note that folders ./items/, ./levels/, and ./actors/ were removed so that the benchmark can successfully execute on used hardware. It reduced the amount of code by rougly 2.5x.*

- Path to the code: shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon (1.9 MB of code)
- Stack Graph built in 45.8s; It has 1,132,110 vertices, 963,522 edges; 128,226 symbols
- Partial Paths Database built in 45.2s
- CFL graph built in 1.9s (6.3s with simplification enabled)
- Generated UCFS grammar file size: 230.7 KB; There are 128,228 rules
- Generated UCFS graph file size: 50.4 MB; Graph has 1,610,791 vertices, 1,442,204 edges
- Generated simplified UCFS graph size: 44.3 MB; Graph has 1,271,983 vertices, 1,274,730 edges

1. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:487:11

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 8ms ± 1ms
  - UCFS: 01:08 ± 6.0s
  - UCFS Simplified: <corrupted> ± 0ms

2. badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:592:14

  - Resolved to 6 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1158:34
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:579:8
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1308:52
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1120:41
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1143:41
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1152:34
  - Stack Graphs: 6ms ± 1ms
  - UCFS: 01:19 ± 22.2s
  - UCFS Simplified: 01:07 ± 2.2s

3. Statistics at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:381:51

  - Resolved to 16 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:9:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:9:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Statistics.java:10:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:12:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:10:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CustomNoteButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:14:48
  - Stack Graphs: 8ms ± 0ms
  - UCFS: 01:30 ± 37.4s
  - UCFS Simplified: 01:08 ± 13.2s

4. removedBadges at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:258:9

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:240:38
  - Stack Graphs: 6ms ± 1ms
  - UCFS: 01:57 ± 47.8s
  - UCFS Simplified: 01:18 ± 26.7s

5. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:259:17

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 3ms ± 1ms
  - UCFS: 02:03 ± 26.6s
  - UCFS Simplified: 01:12 ± 23.4s

6. badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:554:7

  - Resolved to 6 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1120:41
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1308:52
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:548:8
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1158:34
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1152:34
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1143:41
  - Stack Graphs: 7ms ± 1ms
  - UCFS: 01:24 ± 23.6s
  - UCFS Simplified: 01:06 ± 6.6s

7. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:373:11

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 7ms ± 1ms
  - UCFS: 01:30 ± 45.9s
  - UCFS Simplified: 01:11 ± 18.8s

8. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:330:23

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 01:24 ± 23.2s
  - UCFS Simplified: 01:06 ± 4.0s

### Project: [JiaoZi Video Player](https://github.com/lipangit/JiaoZiVideoPlayer)

- Path to the code: JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo (0.1 MB of code)
- Stack Graph built in 2.6s; It has 53,226 vertices, 44,108 edges; 6,986 symbols
- Partial Paths Database built in 864ms
- CFL graph built in 56ms (232ms with simplification enabled)
- Generated UCFS grammar file size: 35.7 KB; There are 6,988 rules
- Generated UCFS graph file size: 2.2 MB; Graph has 75,046 vertices, 65,929 edges
- Generated simplified UCFS graph size: 1.9 MB; Graph has 59,345 vertices, 58,762 edges

1. listView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:31:8

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:15:13
  - Stack Graphs: 4ms ± 0ms
  - UCFS: 5.8s ± 367ms
  - UCFS Simplified: 4.3s ± 554ms

2. top at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:94:74

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:66:89
  - Stack Graphs: 8ms ± 0ms
  - UCFS: 7.3s ± 503ms
  - UCFS Simplified: 5.0s ± 461ms

3. VideoConstant at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:73:34

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\VideoConstant.java:3:13
  - Stack Graphs: 8ms ± 0ms
  - UCFS: 6.1s ± 641ms
  - UCFS Simplified: 4.8s ± 634ms

4. ijkMediaPlayer at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:53:12

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:20:19
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 5.7s ± 264ms
  - UCFS Simplified: 1.3s ± 125ms

5. convertView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:118:75

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:115:47
  - Stack Graphs: 13ms ± 1ms
  - UCFS: 6.1s ± 1.0s
  - UCFS Simplified: 5.0s ± 543ms

6. Glide at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:43:8

  - Resolved to 15 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:15:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:14:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:9:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:14:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:5:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:12:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:8:26
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 6.2s ± 646ms
  - UCFS Simplified: 4.2s ± 531ms

7. mWebView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:84:20

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:21:12
  - Stack Graphs: 9ms ± 0ms
  - UCFS: 10.9s ± 791ms
  - UCFS Simplified: 5.3s ± 544ms

8. R at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdSpeed.java:25:31

  - Resolved to 5 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\MyJzvdStd.java:10:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdShowShareButtonAfterFullscreen.java:9:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdSpeed.java:8:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdMp3.java:7:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:40:20
  - Stack Graphs: 4ms ± 1ms
  - UCFS: 4.3s ± 446ms
  - UCFS Simplified: 4.0s ± 301ms

9. MotionEvent at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\MyJzvdStd.java:44:21

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\MyJzvdStd.java:5:20
  - Stack Graphs: 4ms ± 0ms
  - UCFS: 4.8s ± 387ms
  - UCFS Simplified: 3.9s ± 337ms

10. recyclerView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:31:8

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:14:17
  - Stack Graphs: 4ms ± 1ms
  - UCFS: 6.3s ± 618ms
  - UCFS Simplified: 4.2s ± 442ms

11. ActivityDirectPlay at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:57:52

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:13:13
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 4.4s ± 635ms
  - UCFS Simplified: 4.5s ± 615ms

12. jzvdStd at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:59:19

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:70:16
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 1.2s ± 147ms
  - UCFS Simplified: 476ms ± 151ms

13. Jzvd at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:25:12

  - Resolved to 27 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:15:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewFragmentViewPager.java:13:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:24:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:39:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloadingList.java:6:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:14:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdList.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:7:15
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 4.4s ± 686ms
  - UCFS Simplified: 4.1s ± 621ms

14. savedInstanceState at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:19:23

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:18:45
  - Stack Graphs: 3ms ± 0ms
  - UCFS: 4.3s ± 526ms
  - UCFS Simplified: 4.5s ± 574ms

15. mInflater at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:141:34

  - Resolved to 2 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:140:35
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:92:23
  - Stack Graphs: 12ms ± 1ms
  - UCFS: 6.7s ± 1.1s
  - UCFS Simplified: 5.1s ± 242ms

16. extra at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:161:46

  - Resolved to 2 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:166:77
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:160:78
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 1.2s ± 63ms
  - UCFS Simplified: 1.1s ± 120ms

17. VideoConstant at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:52:61

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\VideoConstant.java:3:13
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 4.3s ± 376ms
  - UCFS Simplified: 4.0s ± 615ms

18. Glide at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:50:8

  - Resolved to 15 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:14:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:9:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:15:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:5:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:14:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:12:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:10:26
  - Stack Graphs: 5ms ± 0ms
  - UCFS: 7.7s ± 982ms
  - UCFS Simplified: 4.3s ± 530ms

19. position at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:128:51

  - Resolved to 4 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:110:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:105:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:115:32
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:150:39
  - Stack Graphs: 13ms ± 1ms
  - UCFS: 5.7s ± 542ms
  - UCFS Simplified: 5.1s ± 483ms

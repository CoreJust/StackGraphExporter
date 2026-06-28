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

### Project: [libgdx new](https://github.com/libgdx/libgdx)

- Path to the code: libgdx/gdx/src/com/badlogic/gdx (3.4 MB of code)
- Stack Graph built in 4.3s; It has 2,448,395 vertices, 2,168,813 edges; 293,282 symbols
- Partial Paths Database built in 8.8s
- CFL graph built in 523ms (2.5s with simplification enabled)
- Generated UCFS grammar file size: 0.4 KB; There are 23,161 rules
- Generated UCFS graph file size: 113.0 MB; Graph has 3,557,998 vertices, 3,278,417 edges
- Generated simplified UCFS graph size: 99.5 MB; Graph has 2,867,087 vertices, 2,886,186 edges

1. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:63:7

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 38.3s ± 777ms
  - UCFS Simplified: 41.4s ± 1.0s

2. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:26:10

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 1.8s ± 102ms
  - UCFS Simplified: 1.8s ± 11ms

3. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:28:8

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.0s ± 578ms
  - UCFS Simplified: 3.2s ± 172ms

4. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:33:8

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.2s ± 196ms
  - UCFS Simplified: 3.5s ± 126ms

5. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:59:42

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 6.2s ± 522ms
  - UCFS Simplified: 6.1s ± 214ms

6. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:32:2

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.5s ± 282ms
  - UCFS Simplified: 3.6s ± 20ms

7. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:75:35

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.0s ± 592ms
  - UCFS Simplified: 3.6s ± 40ms

8. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:34:14

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 4.0s ± 3.4s
  - UCFS Simplified: 3.3s ± 452ms

9. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:154:57

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 39.2s ± 1.7s
  - UCFS Simplified: 48.3s ± 962ms

10. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:551:2

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 35.9s ± 1.5s
  - UCFS Simplified: 39.1s ± 1.4s

11. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:28:8

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.0s ± 227ms
  - UCFS Simplified: 3.2s ± 85ms

12. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:58:8

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 7.5s ± 259ms
  - UCFS Simplified: 5.5s ± 95ms

13. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:56:25

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 41.3s ± 1.2s
  - UCFS Simplified: 41.4s ± 1.5s

14. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:71:8

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFont.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Mesh.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\RenderableSorter.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TideMapLoader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ShaderProgram.java:21:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ModelInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ImmediateModeRenderer20.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\NodeAnimation.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ArraySelection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapLayers.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelInstance.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\AfterAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bresenham2.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\RenderableProvider.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\CameraGroupStrategy.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ParticleControllerInfluencer.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelMaterial.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffectLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ShapeCache.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelCache.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMap.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attribute.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\PointSpriteParticleBatch.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\SelectBox.java:25:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\ObjLoader.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseAnimationController.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\RegionInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\SpotLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffect.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Tree.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffectPool.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\Array.java:13:13
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\DefaultShader.java:33:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmjMapLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\G3dModelLoader.java:24:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Stack.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\ButtonGroup.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleController.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture3D.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\DecalBatch.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseShaderProvider.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\ModelInstanceParticleBatch.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmjMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Table.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupPlug.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ScissorStack.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\BaseShader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParallelArray.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\SimpleOrthoGroupStrategy.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Model.java:36:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\PointLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Cubemap.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelData.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\GLFrameBuffer.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\BSpline.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelAnimation.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Button.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\DistanceFieldFont.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\PluggableGroupStrategy.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTmjMapLoader.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Group.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupStrategy.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Octree.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\SpriteCache.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:35:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BufferedParticleBatch.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PolygonRegionLoader.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\CumulativeDistribution.java:3:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SkinLoader.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\InputMultiplexer.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attributes.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSystem.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\DynamicsInfluencer.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Node.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ResourceData.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\MeshBuilder.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Skin.java:23:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Environment.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ModelBuilder.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEmitter.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TooltipManager.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMapTileSets.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\List.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Renderable.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapObjects.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Material.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSorter.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\GlyphLayout.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\TextureAtlas.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffect.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PixmapPacker.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTiledMapLoader.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\Selection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\AssetLoader.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\DragAndDrop.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\tiles\AnimatedTiledMapTile.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\ParallelAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\TextureArray.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFontCache.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\DefaultRenderableSorter.java:11:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 2.9s ± 91ms
  - UCFS Simplified: 3.4s ± 71ms

15. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:430:67

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\GlyphLayout.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTiledMapLoader.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMap.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\SpotLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\AssetLoader.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelInstance.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\ModelInstanceParticleBatch.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ParticleControllerInfluencer.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffectLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\SimpleOrthoGroupStrategy.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\Selection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\RenderableSorter.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bresenham2.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Group.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseAnimationController.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Cubemap.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTmjMapLoader.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TooltipManager.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelMaterial.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\SpriteCache.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\AfterAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\PointLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\NodeAnimation.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\PluggableGroupStrategy.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\BSpline.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\DecalBatch.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\DefaultRenderableSorter.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ShapeCache.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\TextureArray.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\TextureAtlas.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PolygonRegionLoader.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Environment.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\BaseShader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Skin.java:23:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Stack.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\CumulativeDistribution.java:3:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TideMapLoader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attribute.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\SelectBox.java:25:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Button.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFont.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\DefaultShader.java:33:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Tree.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\GLFrameBuffer.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\List.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSorter.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\DynamicsInfluencer.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelCache.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffect.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ModelInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture3D.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParallelArray.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\DragAndDrop.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\Array.java:13:13
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupStrategy.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\MeshBuilder.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BufferedParticleBatch.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Mesh.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmjMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\G3dModelLoader.java:24:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\ButtonGroup.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFontCache.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffectPool.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PixmapPacker.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ResourceData.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ShaderProgram.java:21:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapLayers.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Model.java:36:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ImmediateModeRenderer20.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMapTileSets.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Material.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\RenderableProvider.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapObjects.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\ObjLoader.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\InputMultiplexer.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseShaderProvider.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Renderable.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\PointSpriteParticleBatch.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmjMapLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupPlug.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\tiles\AnimatedTiledMapTile.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ArraySelection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSystem.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleController.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\ParallelAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\CameraGroupStrategy.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Table.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attributes.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEmitter.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Octree.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SkinLoader.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Node.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\RegionInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ScissorStack.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelAnimation.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelData.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\DistanceFieldFont.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ModelBuilder.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:35:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffect.java:10:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 42.2s ± 2.2s
  - UCFS Simplified: 38.4s ± 776ms

16. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:29:36

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelData.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\BSpline.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffectLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Node.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\PluggableGroupStrategy.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelInstance.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bresenham2.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleController.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\BaseShader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ResourceData.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelAnimation.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\ParallelAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ScissorStack.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ShapeCache.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:35:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Button.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\Selection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMapTileSets.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TooltipManager.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTiledMapLoader.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\AssetLoader.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\PointSpriteParticleBatch.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelMaterial.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParallelArray.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Table.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\RegionInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TideMapLoader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Environment.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSystem.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PolygonRegionLoader.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\TextureArray.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFont.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\RenderableProvider.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ModelBuilder.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffectPool.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\SpotLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ShaderProgram.java:21:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\DefaultShader.java:33:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\InputMultiplexer.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Group.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\RenderableSorter.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseShaderProvider.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Mesh.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Tree.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Renderable.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\tiles\AnimatedTiledMapTile.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupPlug.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmjMapLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attributes.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\MeshBuilder.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\CameraGroupStrategy.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\SelectBox.java:25:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMap.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmjMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ModelInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSorter.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ArraySelection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\ObjLoader.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Material.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\DecalBatch.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffect.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\ButtonGroup.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFontCache.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\PointLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Skin.java:23:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffect.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\DragAndDrop.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\TextureAtlas.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\List.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attribute.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BufferedParticleBatch.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ParticleControllerInfluencer.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ImmediateModeRenderer20.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\NodeAnimation.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\SimpleOrthoGroupStrategy.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\SpriteCache.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEmitter.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTmjMapLoader.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Model.java:36:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\CumulativeDistribution.java:3:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\Array.java:13:13
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PixmapPacker.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\ModelInstanceParticleBatch.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\G3dModelLoader.java:24:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Octree.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\DistanceFieldFont.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Stack.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture3D.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\GlyphLayout.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapLayers.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\AfterAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\DefaultRenderableSorter.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseAnimationController.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapObjects.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelCache.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SkinLoader.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\GLFrameBuffer.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\DynamicsInfluencer.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Cubemap.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupStrategy.java:5:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.2s ± 468ms
  - UCFS Simplified: 3.3s ± 26ms

17. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:25:11

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSorter.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMapTileSets.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleController.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffectPool.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFontCache.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\GLFrameBuffer.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\InputMultiplexer.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\GlyphLayout.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\TextureAtlas.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffect.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ImmediateModeRenderer20.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelAnimation.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\SpriteCache.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\RenderableSorter.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\NodeAnimation.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapObjects.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTiledMapLoader.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Stack.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseShaderProvider.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ScissorStack.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Group.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelMaterial.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\RenderableProvider.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\tiles\AnimatedTiledMapTile.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelInstance.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attribute.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParallelArray.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffectLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\BSpline.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSystem.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TooltipManager.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Tree.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\CumulativeDistribution.java:3:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\ParallelAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ParticleControllerInfluencer.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\ModelInstanceParticleBatch.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Cubemap.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TideMapLoader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ModelInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\G3dModelLoader.java:24:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\DecalBatch.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupStrategy.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\DistanceFieldFont.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffect.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelCache.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\List.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEmitter.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMap.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Skin.java:23:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\ObjLoader.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\Selection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseAnimationController.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\SpotLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\DynamicsInfluencer.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapLayers.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Button.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SkinLoader.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFont.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ArraySelection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Octree.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture3D.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\AfterAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\SelectBox.java:25:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\ButtonGroup.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:35:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\BaseShader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Mesh.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\DefaultShader.java:33:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ShaderProgram.java:21:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PolygonRegionLoader.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Model.java:36:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelData.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ResourceData.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTmjMapLoader.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\AssetLoader.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\DragAndDrop.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Material.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\PointLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupPlug.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\MeshBuilder.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\TextureArray.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\CameraGroupStrategy.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\RegionInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PixmapPacker.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmjMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmjMapLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\SimpleOrthoGroupStrategy.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\DefaultRenderableSorter.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\PointSpriteParticleBatch.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Renderable.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Environment.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BufferedParticleBatch.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ShapeCache.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\Array.java:13:13
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ModelBuilder.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bresenham2.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Table.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\PluggableGroupStrategy.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attributes.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Node.java:10:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 6.1s ± 414ms
  - UCFS Simplified: 6.5s ± 291ms

18. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:39:8

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFont.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\SpriteCache.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\AssetLoader.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\PluggableGroupStrategy.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\AfterAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelMaterial.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\TextureArray.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ArraySelection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Renderable.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attributes.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\TextureAtlas.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParallelArray.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ImmediateModeRenderer20.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelCache.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\BSpline.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupPlug.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\SelectBox.java:25:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Skin.java:23:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapObjects.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SkinLoader.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ParticleControllerInfluencer.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleController.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMap.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSorter.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Tree.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEmitter.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\GLFrameBuffer.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ResourceData.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ShaderProgram.java:21:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PolygonRegionLoader.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Cubemap.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\GlyphLayout.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\NodeAnimation.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSystem.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attribute.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\DistanceFieldFont.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\SimpleOrthoGroupStrategy.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMapTileSets.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Model.java:36:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffect.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\DecalBatch.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\Array.java:13:13
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TideMapLoader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Material.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\DefaultShader.java:33:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Mesh.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Environment.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\RenderableSorter.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Button.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\DynamicsInfluencer.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bresenham2.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ModelBuilder.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelAnimation.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\DragAndDrop.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\ButtonGroup.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\List.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ModelInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\MeshBuilder.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTiledMapLoader.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffectPool.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFontCache.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PixmapPacker.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTmjMapLoader.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\tiles\AnimatedTiledMapTile.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\ParallelAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\ModelInstanceParticleBatch.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Table.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\Selection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\PointSpriteParticleBatch.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelInstance.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffectLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\CumulativeDistribution.java:3:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmjMapLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffect.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Node.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Stack.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmjMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\RegionInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\CameraGroupStrategy.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\SpotLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\InputMultiplexer.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ShapeCache.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Group.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\G3dModelLoader.java:24:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\BaseShader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:35:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseAnimationController.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\ObjLoader.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\DefaultRenderableSorter.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BufferedParticleBatch.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Octree.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupStrategy.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelData.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseShaderProvider.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ScissorStack.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture3D.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapLayers.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\RenderableProvider.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TooltipManager.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\PointLightsAttribute.java:6:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.6s ± 229ms
  - UCFS Simplified: 3.4s ± 11ms

19. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:442:2

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTmjMapLoader.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelCache.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParallelArray.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\ModelInstanceParticleBatch.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SkinLoader.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ShaderProgram.java:21:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Cubemap.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelAnimation.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapLayers.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\SelectBox.java:25:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\SimpleOrthoGroupStrategy.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\InputMultiplexer.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\AfterAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Renderable.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Node.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleController.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\PointSpriteParticleBatch.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ResourceData.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMapTileSets.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelMaterial.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\MeshBuilder.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Table.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Tree.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\ObjLoader.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\DynamicsInfluencer.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffectPool.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PixmapPacker.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupPlug.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attribute.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Button.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\NodeAnimation.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ImmediateModeRenderer20.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\CameraGroupStrategy.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ArraySelection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attributes.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\RenderableProvider.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\BaseShader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\ParallelAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\G3dModelLoader.java:24:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\Selection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Mesh.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffect.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapObjects.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\Array.java:13:13
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\GlyphLayout.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ModelBuilder.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\BSpline.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ModelInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmjMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\ButtonGroup.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\tiles\AnimatedTiledMapTile.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmjMapLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TideMapLoader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFontCache.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\SpriteCache.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ShapeCache.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Octree.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupStrategy.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture3D.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\List.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PolygonRegionLoader.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\PointLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\AssetLoader.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelData.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BufferedParticleBatch.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ScissorStack.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEmitter.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\CumulativeDistribution.java:3:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Material.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Group.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffect.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\TextureAtlas.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\GLFrameBuffer.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelInstance.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ParticleControllerInfluencer.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTiledMapLoader.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFont.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\DecalBatch.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Environment.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:35:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\DistanceFieldFont.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Model.java:36:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseAnimationController.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseShaderProvider.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMap.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\DragAndDrop.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSystem.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSorter.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\DefaultRenderableSorter.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\TextureArray.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\DefaultShader.java:33:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TooltipManager.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\PluggableGroupStrategy.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\RenderableSorter.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\RegionInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bresenham2.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Skin.java:23:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\SpotLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Stack.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffectLoader.java:14:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 40.2s ± 2.9s
  - UCFS Simplified: 41.3s ± 1.9s

20. Array at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:49:8

  - Resolved to 128 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\DynamicsInfluencer.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ResourceData.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SkinLoader.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\GlyphLayout.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ShaderProgram.java:21:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupPlug.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\SoundLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSystem.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\DefaultRenderableSorter.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attributes.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffect.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\List.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\MusicLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Animation.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:35:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParallelArray.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\shapebuilders\RenderableShapeBuilder.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\TextureAtlasLoader.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BillboardParticleBatch.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelNodeAnimation.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\GroupStrategy.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTiledMapLoader.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMapTileSets.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\BitmapFontLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFontCache.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\NodeAnimation.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\RenderableSorter.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\G3dModelLoader.java:24:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture3D.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Actor.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\Selection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ShapeCache.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\InputMultiplexer.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\CumulativeDistribution.java:3:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\SpotLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ShaderProgramLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\TextureAtlas.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Model.java:36:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bresenham2.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\SelectBox.java:25:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleSorter.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Tree.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelData.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffect.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\PointLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\PluggableGroupStrategy.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\DecalBatch.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ParticleEffectLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\I18NBundleLoader.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelInstance.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseShaderProvider.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\BaseTmjMapLoader.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\AssetLoader.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Bezier.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Skin.java:23:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\ParallelAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelBatch.java:13:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\Node.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TiledMap.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Octree.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\PointSpriteParticleBatch.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\SpriteCache.java:19:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\ModelBuilder.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PolygonRegionLoader.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\MeshBuilder.java:27:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\DragAndDrop.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TooltipManager.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Attribute.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Cubemap.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEffectPool.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\RegionInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ScissorStack.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Button.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Material.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\TextField.java:29:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\Intersector.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\actions\AfterAction.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\ParticleEmitter.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmxMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\BufferedParticleBatch.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\utils\ArraySelection.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ModelInfluencer.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\batches\ModelInstanceParticleBatch.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Texture.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\ButtonGroup.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TmjMapLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelAnimation.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\RenderableProvider.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetLoadingTask.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\utils\BaseAnimationController.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\AtlasTmjMapLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\PixmapLoader.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Group.java:11:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\TideMapLoader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\tiled\tiles\AnimatedTiledMapTile.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\attributes\DirectionalLightsAttribute.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleController.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\RepeatablePolygonSprite.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\influencers\ParticleControllerInfluencer.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\BaseShader.java:20:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\math\BSpline.java:4:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\GLFrameBuffer.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\SimpleOrthoGroupStrategy.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\TextureArray.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\ModelCache.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Renderable.java:8:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\Mesh.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapObjects.java:6:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\PixmapPacker.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\CubemapLoader.java:15:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\loader\ObjLoader.java:30:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Stack.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\utils\Array.java:13:13
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\model\data\ModelMaterial.java:5:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\BitmapFont.java:17:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\particles\ParticleEffectLoader.java:14:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\decals\CameraGroupStrategy.java:10:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\glutils\ImmediateModeRenderer20.java:9:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g2d\DistanceFieldFont.java:7:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\Stage.java:26:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\shaders\DefaultShader.java:33:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\graphics\g3d\Environment.java:12:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\loaders\ModelLoader.java:16:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\scenes\scene2d\ui\Table.java:18:30
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\maps\MapLayers.java:6:30
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 3.1s ± 594ms
  - UCFS Simplified: 3.5s ± 2ms


### Project: [libgdx old](https://github.com/libgdx/libgdx)

- Path to the code: C:\Users\egor2\OneDrive\Рабочий стол\Доки для ВУЗа\Thesis\tree-sitter\sources\libgdx\gdx\src\com\badlogic\gdx (3.2 MB of code)
- Stack Graph built in 01:18; It has 2,448,395 vertices, 2,168,813 edges; 293,282 symbols
- Partial Paths Database built in 01:57
- CFL graph built in 3.8s (14.0s with simplification enabled)
- Generated UCFS grammar file size: 0.4 KB; There are 23,161 rules
- Generated UCFS graph file size: 113.0 MB; Graph has 3,557,998 vertices, 3,278,417 edges
- Generated simplified UCFS graph size: 99.5 MB; Graph has 2,867,087 vertices, 2,886,186 edges

1. assetsByType at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:285:6

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:284:41
  - Stack Graphs: 14ms ± 2ms
  - UCFS: 01:34 ± 36.5s
  - UCFS Simplified: 01:51 ± 8.6s

2. tasks at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:338:22

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:63:31
  - Stack Graphs: 11ms ± 1ms
  - UCFS: 01:48 ± 8.1s
  - UCFS Simplified: 01:46 ± 4.3s

3. object at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:265:8

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:264:11
  - Stack Graphs: 11ms ± 1ms
  - UCFS: 01:49 ± 2.2s
  - UCFS Simplified: 01:48 ± 3.5s

4. i at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:339:36

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:338:11
  - Stack Graphs: 12ms ± 0ms
  - UCFS: 01:47 ± 13.4s
  - UCFS Simplified: 01:46 ± 9.6s

5. fileName at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:193:45

  - Resolved to 18 definitions:
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:738:48
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:189:41
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:174:46
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:283:46
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:733:59
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:311:43
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:164:46
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:124:45
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:295:69
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:119:39
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:316:43
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:687:51
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:694:52
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:493:43
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:138:45
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:277:46
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:114:39
    - ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:409:41
  - Stack Graphs: 3ms ± 0ms
  - UCFS: 01:36 ± 1.2s
  - UCFS Simplified: 01:58 ± 4.4s

6. loadQueue at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:331:26

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:60:30
  - Stack Graphs: 10ms ± 1ms
  - UCFS: 01:21 ± 3.3s
  - UCFS Simplified: 01:51 ± 4.5s

7. tasks at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:192:34

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:63:31
  - Stack Graphs: 10ms ± 1ms
  - UCFS: 01:46 ± 8.4s
  - UCFS Simplified: 01:50 ± 7.0s

8. foundIndex at ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:207:4

  - Resolved to ..\sources\libgdx\gdx\src\com\badlogic\gdx\assets\AssetManager.java:204:6
  - Stack Graphs: 11ms ± 1ms
  - UCFS: 01:40 ± 2.5s
  - UCFS Simplified: 01:59 ± 3.0s


### Project: [JsonPath new](https://github.com/json-path/JsonPath)

- Path to the code: JsonPath\json-path\src\main\java\com\jayway\jsonpath (0.4 MB of code)
- Stack Graph built in 506ms; It has 178,914 vertices, 153,255 edges; 23,071 symbols
- Partial Paths Database built in 371ms
- CFL graph built in 30ms (132ms with simplification enabled)
- Generated UCFS grammar file size: 0.4 KB; There are 2,817 rules
- Generated UCFS graph file size: 7.4 MB; Graph has 253,715 vertices, 228,057 edges
- Generated simplified UCFS graph size: 6.5 MB; Graph has 200,411 vertices, 200,424 edges

1. Configuration at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:11:13

  - Resolved to 20 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JacksonMappingProvider.java:5:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicateContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\latebinding\PathLateBindingValue.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\CompiledPath.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\ParseContextImpl.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\Path.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\PathRef.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonSmartMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JakartaMappingProvider.java:22:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonOrgMappingProvider.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\TapestryMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\GsonMappingProvider.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNodes.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\JsonContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\Jackson3MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:11:13
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 16ms ± 1ms
  - UCFS Simplified: 19ms ± 2ms

2. Defaults at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:16:48

  - Resolved to 2 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:167:21
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\DefaultsImpl.java:2:41
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 352ms ± 3ms
  - UCFS Simplified: 361ms ± 22ms

3. Defaults at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:20:19

  - Resolved to 2 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:167:21
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\DefaultsImpl.java:2:41
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 460ms ± 191ms
  - UCFS Simplified: 374ms ± 3ms

4. Option at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:30:22

  - Resolved to 10 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Option.java:3:12
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArrayPathToken.java:4:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PathToken.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\JsonContext.java:8:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ScanPathToken.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PropertyPathToken.java:4:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:5:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\DefaultsImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNodes.java:8:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\WildcardPathToken.java:5:27
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 574ms ± 13ms
  - UCFS Simplified: 551ms ± 237ms

5. Collections at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:41:35

  - Resolved to 6 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:9:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\WildcardPathToken.java:3:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\PathFunctionFactory.java:16:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:15:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArrayIndexOperation.java:6:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JettisonProvider.java:8:17
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 585ms ± 12ms
  - UCFS Simplified: 590ms ± 4ms

6. Configuration at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:70:11

  - Resolved to 20 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\latebinding\PathLateBindingValue.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonSmartMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\PathRef.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JsonOrgMappingProvider.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\GsonMappingProvider.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\ParseContextImpl.java:2:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNodes.java:6:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\Jackson3MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JacksonMappingProvider.java:5:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\TapestryMappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicateContextImpl.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\CompiledPath.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JakartaMappingProvider.java:22:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\MappingProvider.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:11:13
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\Path.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:3:27
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\JsonContext.java:3:27
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 597ms ± 5ms
  - UCFS Simplified: 591ms ± 22ms

7. Collection at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Configuration.java:119:16

  - Resolved to 21 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:14:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\AbstractJsonProvider.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JettisonProvider.java:7:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JacksonJsonNodeJsonProvider.java:17:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Filter.java:6:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\PathRef.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JakartaJsonProvider.java:27:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PathCompiler.java:12:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\TapestryJsonProvider.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JsonProvider.java:6:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PathTokenFactory.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\mapper\JakartaMappingProvider.java:13:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\LogicalExpressionNode.java:5:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\GsonJsonProvider.java:18:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\Jackson3JsonNodeJsonProvider.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\PredicatePathToken.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ScanPathToken.java:7:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Criteria.java:12:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\spi\json\JsonOrgJsonProvider.java:14:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\Parameter.java:7:17
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 594ms ± 9ms
  - UCFS Simplified: 646ms ± 18ms

8. ValueNode at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Criteria.java:64:28

  - Resolved to 2 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Criteria.java:7:43
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ValueNode.java:12:22
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 70ms ± 58ms
  - UCFS Simplified: 104ms ± 1ms

9. shouldExist at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Criteria.java:238:52

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\Criteria.java:235:35
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 38ms ± 1ms
  - UCFS Simplified: 40ms ± 3ms

10. position at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:231:36

  - Resolved to 2 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:22:16
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\CharacterIndex.java:74:15
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 102ms ± 49ms
  - UCFS Simplified: 105ms ± 13ms

11. left at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ExpressionNode.java:17:34

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\filter\ExpressionNode.java:6:118
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 36ms ± 3ms
  - UCFS Simplified: 41ms ± 4ms

12. EvaluationContext at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\sequence\AbstractSequenceAggregation.java:38:41

  - Resolved to 15 definitions:
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\text\Length.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\EvaluationContext.java:8:17
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\PathFunction.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\text\Concatenate.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\Parameter.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\CompiledPath.java:5:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\sequence\Last.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\numeric\AbstractAggregation.java:3:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\PassthruPathFunction.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\sequence\Index.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\json\KeySetFunction.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\sequence\First.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\sequence\AbstractSequenceAggregation.java:3:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\function\json\Append.java:2:36
    - ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\EvaluationContextImpl.java:8:36
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 33ms ± 2ms
  - UCFS Simplified: 37ms ± 5ms

13. i at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:39

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:17
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 54ms ± 1ms
  - UCFS Simplified: 55ms ± 17ms

14. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:31

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 82ms ± 65ms
  - UCFS Simplified: 78ms ± 13ms

15. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:31

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 52ms ± 2ms
  - UCFS Simplified: 55ms ± 11ms

16. i at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:27

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:17
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 75ms ± 6ms
  - UCFS Simplified: 82ms ± 5ms

17. i at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:27

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:17
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 54ms ± 2ms
  - UCFS Simplified: 60ms ± 2ms

18. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:21

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 78ms ± 9ms
  - UCFS Simplified: 99ms ± 95ms

19. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:48:21

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 55ms ± 1ms
  - UCFS Simplified: 54ms ± 1ms

20. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:45:35

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 76ms ± 9ms
  - UCFS Simplified: 83ms ± 9ms

21. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:45:35

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 54ms ± 6ms
  - UCFS Simplified: 54ms ± 1ms

22. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:45:27

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 75ms ± 11ms
  - UCFS Simplified: 77ms ± 7ms

23. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:45:27

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 51ms ± 1ms
  - UCFS Simplified: 52ms ± 1ms

24. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:45:12

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 75ms ± 3ms
  - UCFS Simplified: 75ms ± 7ms

25. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:45:12

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 54ms ± 1ms
  - UCFS Simplified: 51ms ± 3ms

26. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:43:114

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 75ms ± 2ms
  - UCFS Simplified: 76ms ± 10ms

27. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:43:114

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 51ms ± 1ms
  - UCFS Simplified: 54ms ± 1ms

28. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:43:108

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 73ms ± 4ms
  - UCFS Simplified: 76ms ± 2ms

29. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:43:108

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 52ms ± 2ms
  - UCFS Simplified: 55ms ± 2ms

30. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:43:100

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 73ms ± 3ms
  - UCFS Simplified: 76ms ± 10ms

31. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:43:100

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 51ms ± 1ms
  - UCFS Simplified: 54ms ± 3ms

32. logger at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:43:8

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:9:32
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 75ms ± 2ms
  - UCFS Simplified: 77ms ± 6ms

33. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:41:27

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 73ms ± 4ms
  - UCFS Simplified: 76ms ± 6ms

34. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:41:27

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 52ms ± 1ms
  - UCFS Simplified: 54ms ± 2ms

35. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:41:8

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 54ms ± 1ms
  - UCFS Simplified: 56ms ± 1ms

36. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:41:8

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 73ms ± 2ms
  - UCFS Simplified: 74ms ± 6ms

37. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:39:28

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 77ms ± 3ms
  - UCFS Simplified: 74ms ± 10ms

38. from at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:39:28

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:36:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 64ms ± 34ms
  - UCFS Simplified: 55ms ± 3ms

39. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:39:19

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 77ms ± 8ms
  - UCFS Simplified: 76ms ± 6ms

40. length at ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:39:19

  - Resolved to ..\sources\JsonPath\json-path\src\main\java\com\jayway\jsonpath\internal\path\ArraySliceToken.java:35:12
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 59ms ± 5ms
  - UCFS Simplified: 58ms ± 2ms


### Project: [Shattered Pixel Dungeon](https://github.com/00-Evan/shattered-pixel-dungeon)

*Note that folders ./items/, ./levels/, and ./actors/ were removed so that the benchmark can successfully execute on used hardware. It reduced the amount of code by rougly 2.5x.*

- Path to the code: shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon (1.9 MB of code)
- Stack Graph built in 2.1s; It has 1,132,110 vertices, 963,522 edges; 128,226 symbols
- Partial Paths Database built in 3.1s
- CFL graph built in 214ms (996ms with simplification enabled)
- Generated UCFS grammar file size: 0.4 KB; There are 12,361 rules
- Generated UCFS graph file size: 48.7 MB; Graph has 1,610,791 vertices, 1,442,204 edges
- Generated simplified UCFS graph size: 42.6 MB; Graph has 1,271,983 vertices, 1,274,730 edges

1. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1091:6

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 15.6s ± 590ms
  - UCFS Simplified: 16.1s ± 2.2s

2. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:938:66

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 15.8s ± 1.5s
  - UCFS Simplified: 14.0s ± 814ms

3. Messages at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Assets.java:82:21

  - Resolved to 105 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuest.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:18:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Rankings.java:19:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\ChangesScene.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:20:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:56:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ExitButton.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoBuff.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_3_X_Changes.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Assets.java:82:21
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:21:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_9_X_Changes.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RenderedTextBlock.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\StartScene.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:22:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:24:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_5_X_Changes.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:19:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:97:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Document.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\GamesInProgress.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CustomNoteButton.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\PixelScene.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\ChangeButton.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoMob.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:56:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:50:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndError.java:4:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:22:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoSubclass.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\DM200Sprite.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Mageroyal.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemJournalButton.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Bestiary.java:122:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHeroInfo.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SupporterScene.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\DM201Sprite.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoArmorAbility.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSettings.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:23:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:15:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndKeyBindings.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndVictoryCongrats.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:36:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_4_X_Changes.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\NewsScene.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:16:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v3_X_Changes.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\utils\GLog.java:4:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndDocument.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_7_X_Changes.java:27:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSupportPrompt.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v1_X_Changes.java:18:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChallenges.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v2_X_Changes.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\messages\Messages.java:20:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_2_X_Changes.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndDailies.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:25:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_6_X_Changes.java:33:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:27:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\IconTitle.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:16:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_8_X_Changes.java:21:57
  - Stack Graphs: 1ms ± 1ms
  - UCFS: 01:20 ± 2.5s
  - UCFS Simplified: 01:23 ± 1.9s

4. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1001:7

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 18.8s ± 942ms
  - UCFS Simplified: 13.9s ± 699ms

5. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:428:54

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 14.2s ± 796ms
  - UCFS Simplified: 16.2s ± 578ms

6. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:938:22

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 16.2s ± 579ms
  - UCFS Simplified: 15.7s ± 725ms

7. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:941:20

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 14.9s ± 849ms
  - UCFS Simplified: 15.6s ± 491ms

8. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:673:31

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 15.9s ± 467ms
  - UCFS Simplified: 15.8s ± 762ms

9. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:936:7

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 15.8s ± 586ms
  - UCFS Simplified: 16.1s ± 811ms

10. Messages at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1129:12

  - Resolved to 104 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChallenges.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v3_X_Changes.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHeroInfo.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_6_X_Changes.java:33:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndDocument.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Mageroyal.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:18:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:23:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:56:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\StartScene.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\ChangeButton.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\NewsScene.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemJournalButton.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoArmorAbility.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\ChangesScene.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CustomNoteButton.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Rankings.java:19:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:56:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:20:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:25:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v1_X_Changes.java:18:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ExitButton.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:22:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndDailies.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuest.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoMob.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_7_X_Changes.java:27:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Bestiary.java:122:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_5_X_Changes.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v2_X_Changes.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RenderedTextBlock.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:24:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:50:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SupporterScene.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:27:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_3_X_Changes.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\DM201Sprite.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_8_X_Changes.java:21:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:15:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:22:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSettings.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\GamesInProgress.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\messages\Messages.java:20:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_9_X_Changes.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Document.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSupportPrompt.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoSubclass.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:97:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:16:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\utils\GLog.java:4:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_2_X_Changes.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\PixelScene.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\DM200Sprite.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:21:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndError.java:4:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_4_X_Changes.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndVictoryCongrats.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:16:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\IconTitle.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:36:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:19:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndKeyBindings.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoBuff.java:5:57
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 14.3s ± 714ms
  - UCFS Simplified: 16.1s ± 754ms

11. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:974:7

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 15.7s ± 590ms
  - UCFS Simplified: 16.2s ± 750ms

12. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:809:37

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 14.9s ± 780ms
  - UCFS Simplified: 16.0s ± 560ms

13. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:830:40

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
  - Stack Graphs: 0ms ± 1ms
  - UCFS: 15.8s ± 602ms
  - UCFS Simplified: 16.2s ± 694ms

14. Messages at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1137:11

  - Resolved to 104 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_9_X_Changes.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\PixelScene.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SupporterScene.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoArmorAbility.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:24:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RenderedTextBlock.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:36:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Mageroyal.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\GamesInProgress.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:27:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_3_X_Changes.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuest.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:19:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\StartScene.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v2_X_Changes.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChallenges.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_8_X_Changes.java:21:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:97:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:22:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\NewsScene.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoSubclass.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\DM200Sprite.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:21:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_2_X_Changes.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:56:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\IconTitle.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v3_X_Changes.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Bestiary.java:122:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:50:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_7_X_Changes.java:27:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndError.java:4:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ExitButton.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndDailies.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndDocument.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Rankings.java:19:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:11:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:15:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_5_X_Changes.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:56:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSettings.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSupportPrompt.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndVictoryCongrats.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:25:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_6_X_Changes.java:33:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\messages\Messages.java:20:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemJournalButton.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:10:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Document.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CustomNoteButton.java:14:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHeroInfo.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:7:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:16:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:20:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\ChangeButton.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndKeyBindings.java:6:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:22:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\utils\GLog.java:4:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoMob.java:5:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\DM201Sprite.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\ChangesScene.java:8:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v0_4_X_Changes.java:9:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:12:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\changelist\v1_X_Changes.java:18:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:16:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:13:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:18:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:23:57
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoBuff.java:5:57
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 15.2s ± 530ms
  - UCFS Simplified: 15.8s ± 688ms

15. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:941:64

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 19.3s ± 890ms
  - UCFS Simplified: 12.9s ± 691ms

16. Dungeon at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:673:7

  - Resolved to 118 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\ConeAOE.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUseItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndCombo.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BossHealthBar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\HeroSelectScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Plant.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventorySlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\CheckedCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\CellSelector.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SkeletonSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\KeyDisplay.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Toolbar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GooSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGameInProgress.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Firebloom.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sorrowmoss.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickRecipe.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\TitleScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\CustomTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Compass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBlacksmith.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\NecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\RatKingSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTalent.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Catalog.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndClericSpells.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndQuickBag.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\mechanics\Ballistica.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Surprise.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Fadeleaf.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MirrorSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\WelcomeScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoPlant.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndBag.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Icecap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CurrencyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\DangerIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\BlobEmitter.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalSentrySprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndEnergizeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\InventoryPane.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\Icons.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\RightClickMenu.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\WindParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\RankingsScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\journal\Notes.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndWandmaker.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ActionIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndSadGhost.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\JournalScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Sungrass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Pushing.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\LootIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\SpectralWallParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\FogOfWar.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BusyIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\BlandfruitBush.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\TenguSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTileSheet.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndTradeItem.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Earthroot.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Rotberry.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CrystalSpireSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ItemSlot.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ScorpioSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Swap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\GridTileMap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndMonkAbilities.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\LeafParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\CharSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Wound.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\MenuPane.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\BuffIndicator.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndGame.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\UndeadSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\FloatingText.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Swiftthistle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\HeroSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\BlacksmithSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\RaisedTerrainTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoCell.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\ResumeIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Starflower.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\LotusSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\GnollTricksterSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndImp.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndInfoTrap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\WallBlockingTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\AttackIndicator.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\particles\FlowParticle.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\SurfaceScene.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\FungalCoreSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\plants\Blindweed.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\ItemSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\MissileSprite.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\effects\Ripple.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndUpgrade.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\DungeonWallsTilemap.java:4:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Dungeon.java:80:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\sprites\SpectralNecromancerSprite.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\tiles\TerrainFeaturesTilemap.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AmuletScene.java:7:48
  - Stack Graphs: 0ms ± 1ms
  - UCFS: 15.8s ± 567ms
  - UCFS Simplified: 15.0s ± 696ms


### Project: [Shattered Pixel Dungeon old](https://github.com/00-Evan/shattered-pixel-dungeon)

- Path to the code: C:\Users\egor2\OneDrive\Рабочий стол\Доки для ВУЗа\Thesis\tree-sitter\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon (1.9 MB of code)
- Stack Graph built in 29.9s; It has 1,132,110 vertices, 963,522 edges; 128,226 symbols
- Partial Paths Database built in 35.3s
- CFL graph built in 1.5s (5.2s with simplification enabled)
- Generated UCFS grammar file size: 0.4 KB; There are 12,361 rules
- Generated UCFS graph file size: 48.7 MB; Graph has 1,610,791 vertices, 1,442,204 edges
- Generated simplified UCFS graph size: 42.6 MB; Graph has 1,271,983 vertices, 1,274,730 edges

1. badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:592:14

  - Resolved to 6 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1152:34
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1143:41
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1158:34
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1120:41
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:579:8
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1308:52
  - Stack Graphs: 6ms ± 0ms
  - UCFS: 35.4s ± 2.2s
  - UCFS Simplified: 37.4s ± 3.7s

2. Statistics at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:381:51

  - Resolved to 16 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\CustomNoteButton.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\InterlevelScene.java:9:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\GameScene.java:14:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Statistics.java:10:13
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndScoreBreakdown.java:6:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentButton.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\StatusPane.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseSubclass.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndHero.java:7:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndChooseAbility.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndResurrect.java:5:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\QuickSlotButton.java:8:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndJournal.java:9:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\windows\WndRanking.java:12:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\scenes\AlchemyScene.java:10:48
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\ui\TalentsPane.java:6:48
  - Stack Graphs: 7ms ± 1ms
  - UCFS: 36.1s ± 1.3s
  - UCFS Simplified: 36.8s ± 1.3s

3. badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:554:7

  - Resolved to 6 definitions:
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1143:41
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1152:34
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1158:34
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1308:52
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:548:8
    - ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:1120:41
  - Stack Graphs: 6ms ± 0ms
  - UCFS: 37.5s ± 594ms
  - UCFS Simplified: 36.0s ± 3.5s

4. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:259:17

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 35.3s ± 2.1s
  - UCFS Simplified: 43.0s ± 655ms

5. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:487:11

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 6ms ± 0ms
  - UCFS: 36.6s ± 2.2s
  - UCFS Simplified: 32.9s ± 1.8s

6. removedBadges at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:258:9

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:240:38
  - Stack Graphs: 6ms ± 0ms
  - UCFS: 37.3s ± 1.7s
  - UCFS Simplified: 36.4s ± 1.3s

7. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:330:23

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 2ms ± 0ms
  - UCFS: 32.4s ± 705ms
  - UCFS Simplified: 34.0s ± 4.3s

8. Badge at ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:373:11

  - Resolved to ..\sources\shattered-pixel-dungeon\core\src\main\java\com\shatteredpixel\shatteredpixeldungeon\Badges.java:47:13
  - Stack Graphs: 6ms ± 1ms
  - UCFS: 32.8s ± 446ms
  - UCFS Simplified: 36.3s ± 978ms


### Project: [JiaoZi Video Player new](https://github.com/lipangit/JiaoZiVideoPlayer)

- Path to the code: JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo (0.1 MB of code)
- Stack Graph built in 260ms; It has 53,226 vertices, 44,108 edges; 6,986 symbols
- Partial Paths Database built in 82ms
- CFL graph built in 8ms (34ms with simplification enabled)
- Generated UCFS grammar file size: 0.4 KB; There are 1,513 rules
- Generated UCFS graph file size: 2.0 MB; Graph has 75,046 vertices, 65,929 edges
- Generated simplified UCFS graph size: 1.8 MB; Graph has 59,345 vertices, 58,762 edges

1. AppCompatActivity at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:28:33

  - Resolved to 19 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleView.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewFragmentViewPager.java:7:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloadingList.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListView.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:10:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:6:30
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 6ms ± 0ms
  - UCFS Simplified: 6ms ± 1ms

2. AppCompatActivity at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:28:33

  - Resolved to 19 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewFragmentViewPager.java:7:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloadingList.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:6:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListView.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:5:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:4:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:10:30
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleView.java:4:30
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 6ms ± 0ms
  - UCFS Simplified: 6ms ± 1ms

3. Nullable at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:34:29

  - Resolved to 17 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleView.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:4:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListView.java:4:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloadingList.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:8:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:4:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:4:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:3:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:5:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:3:34
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 133ms ± 16ms
  - UCFS Simplified: 134ms ± 8ms

4. R at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:43:32

  - Resolved to 5 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdMp3.java:7:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdShowShareButtonAfterFullscreen.java:9:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdSpeed.java:8:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\MyJzvdStd.java:10:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:40:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 138ms ± 17ms
  - UCFS Simplified: 140ms ± 15ms

5. map at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:48:8

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:44:22
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 136ms ± 11ms
  - UCFS Simplified: 125ms ± 13ms

6. mSensorManager at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:71:8

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:31:18
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 144ms ± 23ms
  - UCFS Simplified: 134ms ± 10ms

7. e at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:135:12

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:134:27
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 163ms ± 136ms
  - UCFS Simplified: 127ms ± 13ms

8. Jzvd at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:36:8

  - Resolved to 27 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:15:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdList.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:24:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloadingList.java:6:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewFragmentViewPager.java:13:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:39:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:14:15
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 29ms ± 2ms
  - UCFS Simplified: 31ms ± 1ms

9. Jzvd at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:90:32

  - Resolved to 27 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:14:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:15:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloadingList.java:6:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:24:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewFragmentViewPager.java:13:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:39:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdList.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:16:15
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 133ms ± 54ms
  - UCFS Simplified: 129ms ± 15ms

10. Toast at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:71:8

  - Resolved to 5 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:7:22
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:7:22
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdShowShareButtonAfterFullscreen.java:6:22
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:8:22
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:13:22
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 184ms ± 29ms
  - UCFS Simplified: 149ms ± 27ms

11. time at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:140:35

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:138:28
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 47ms ± 2ms
  - UCFS Simplified: 47ms ± 3ms

12. VideoConstant at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\VideoConstant.java:3:13

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\VideoConstant.java:3:13
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 6ms ± 0ms
  - UCFS Simplified: 6ms ± 4ms

13. listView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:61:15

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:16:13
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 38ms ± 3ms
  - UCFS Simplified: 40ms ± 2ms

14. lastVisibleItem at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:90

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:48:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

15. lastVisibleItem at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:90

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:48:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

16. currentPlayPosition at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:67

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:49:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

17. currentPlayPosition at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:67

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:49:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

18. firstVisibleItem at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:47

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:44:55
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

19. firstVisibleItem at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:47

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:44:55
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

20. currentPlayPosition at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:25

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:49:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

21. currentPlayPosition at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:53:25

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:49:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

22. currentPlayPosition at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:52:20

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:49:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

23. currentPlayPosition at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:52:20

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:49:20
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

24. visibleItemCount at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:48:57

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:44:77
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

25. visibleItemCount at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:48:57

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:44:77
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

26. firstVisibleItem at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:48:38

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:44:55
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

27. firstVisibleItem at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:48:38

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:44:55
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 0ms ± 0ms
  - UCFS Simplified: 0ms ± 0ms

28. AbsListView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:37:41

  - Resolved to 3 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:8:22
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:9:22
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:8:22
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 32ms ± 1ms
  - UCFS Simplified: 28ms ± 1ms

29. listView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:37:8

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:16:13
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 37ms ± 4ms
  - UCFS Simplified: 37ms ± 3ms

30. index at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:36:42

  - Resolved to 2 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:19:37
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:17:8
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 38ms ± 3ms
  - UCFS Simplified: 40ms ± 3ms

31. index at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:36:42

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:19:37
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 30ms ± 2ms
  - UCFS Simplified: 32ms ± 2ms

32. VideoConstant at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:36:16

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\VideoConstant.java:3:13
  - Stack Graphs: 0ms ± 0ms
  - UCFS: 39ms ± 2ms
  - UCFS Simplified: 38ms ± 2ms


### Project: [JiaoZi Video Player old](https://github.com/lipangit/JiaoZiVideoPlayer)

- Path to the code: C:\Users\egor2\OneDrive\Рабочий стол\Доки для ВУЗа\Thesis\tree-sitter\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo (0.1 MB of code)
- Stack Graph built in 2.3s; It has 53,226 vertices, 44,108 edges; 6,986 symbols
- Partial Paths Database built in 804ms
- CFL graph built in 53ms (255ms with simplification enabled)
- Generated UCFS grammar file size: 0.4 KB; There are 1,513 rules
- Generated UCFS graph file size: 2.0 MB; Graph has 75,046 vertices, 65,929 edges
- Generated simplified UCFS graph size: 1.8 MB; Graph has 59,345 vertices, 58,762 edges

1. listView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:31:8

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:15:13
  - Stack Graphs: 3ms ± 0ms
  - UCFS: 387ms ± 41ms
  - UCFS Simplified: 336ms ± 47ms

2. top at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:94:74

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:66:89
  - Stack Graphs: 8ms ± 0ms
  - UCFS: 409ms ± 35ms
  - UCFS Simplified: 390ms ± 102ms

3. ijkMediaPlayer at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:53:12

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:20:19
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 163ms ± 26ms
  - UCFS Simplified: 154ms ± 22ms

4. VideoConstant at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:73:34

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\VideoConstant.java:3:13
  - Stack Graphs: 8ms ± 0ms
  - UCFS: 426ms ± 41ms
  - UCFS Simplified: 372ms ± 40ms

5. convertView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:118:75

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:115:47
  - Stack Graphs: 12ms ± 1ms
  - UCFS: 497ms ± 44ms
  - UCFS Simplified: 504ms ± 54ms

6. MotionEvent at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\MyJzvdStd.java:44:21

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\MyJzvdStd.java:5:20
  - Stack Graphs: 3ms ± 0ms
  - UCFS: 512ms ± 41ms
  - UCFS Simplified: 510ms ± 66ms

7. recyclerView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:31:8

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:14:17
  - Stack Graphs: 3ms ± 1ms
  - UCFS: 392ms ± 22ms
  - UCFS Simplified: 361ms ± 46ms

8. R at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdSpeed.java:25:31

  - Resolved to 5 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdMp3.java:7:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdShowShareButtonAfterFullscreen.java:9:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\MyJzvdStd.java:10:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:40:20
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdSpeed.java:8:20
  - Stack Graphs: 3ms ± 0ms
  - UCFS: 508ms ± 51ms
  - UCFS Simplified: 521ms ± 62ms

9. mWebView at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:84:20

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:21:12
  - Stack Graphs: 8ms ± 1ms
  - UCFS: 423ms ± 39ms
  - UCFS Simplified: 459ms ± 26ms

10. Glide at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:43:8

  - Resolved to 15 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:9:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:15:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:12:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:5:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:14:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:14:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:10:26
  - Stack Graphs: 1ms ± 1ms
  - UCFS: 331ms ± 53ms
  - UCFS Simplified: 330ms ± 63ms

11. jzvdStd at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:59:19

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:70:16
  - Stack Graphs: 0ms ± 1ms
  - UCFS: 117ms ± 10ms
  - UCFS Simplified: 116ms ± 12ms

12. extra at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:161:46

  - Resolved to 2 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:160:78
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:166:77
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 146ms ± 20ms
  - UCFS Simplified: 143ms ± 12ms

13. mInflater at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:141:34

  - Resolved to 2 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:92:23
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:140:35
  - Stack Graphs: 12ms ± 0ms
  - UCFS: 473ms ± 82ms
  - UCFS Simplified: 516ms ± 45ms

14. savedInstanceState at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:19:23

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:18:45
  - Stack Graphs: 3ms ± 0ms
  - UCFS: 368ms ± 57ms
  - UCFS Simplified: 360ms ± 65ms

15. VideoConstant at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:52:61

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\VideoConstant.java:3:13
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 331ms ± 39ms
  - UCFS Simplified: 344ms ± 66ms

16. Jzvd at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdList.java:26:27

  - Resolved to 27 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:15:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewFragmentViewPager.java:13:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaExo.java:39:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:16:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityDirectPlay.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewNormal.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:9:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewRecyclerView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:24:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloadingList.java:6:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomJzvd\JzvdStdList.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaIjk.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUIBigChange.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:7:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:10:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\CustomMedia\JZMediaSystemAssertFolder.java:14:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:11:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleView.java:12:15
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:16:15
  - Stack Graphs: 4ms ± 0ms
  - UCFS: 502ms ± 86ms
  - UCFS Simplified: 450ms ± 83ms

17. FragmentDemo at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewFragmentViewPager.java:31:29

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\FragmentDemo.java:14:13
  - Stack Graphs: 1ms ± 0ms
  - UCFS: 334ms ± 37ms
  - UCFS Simplified: 345ms ± 69ms

18. Glide at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:50:8

  - Resolved to 15 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindow.java:9:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityPreloading.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiExtendsNormal.java:5:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerViewTiny.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApi.java:15:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityMain.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterRecyclerView.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiOrientation.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityWebView.java:12:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiCustomMedia.java:10:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\AdapterListView.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiRotationVideoSize.java:8:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityApiUISmallChange.java:7:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:14:26
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:14:26
  - Stack Graphs: 4ms ± 0ms
  - UCFS: 356ms ± 43ms
  - UCFS Simplified: 385ms ± 76ms

19. position at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:128:51

  - Resolved to 4 definitions:
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:115:32
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:150:39
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:105:34
    - ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityListViewMultiHolder.java:110:34
  - Stack Graphs: 12ms ± 0ms
  - UCFS: 484ms ± 84ms
  - UCFS Simplified: 496ms ± 80ms

20. TextHolder at ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:126:41

  - Resolved to ..\sources\JiaoZiVideoPlayer\app\src\main\java\cn\jzvd\demo\ActivityTinyWindowRecycleViewMultiHolder.java:151:14
  - Stack Graphs: 3ms ± 0ms
  - UCFS: 374ms ± 47ms
  - UCFS Simplified: 370ms ± 55ms

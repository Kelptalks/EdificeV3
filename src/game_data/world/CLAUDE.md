# world/

Root file: `world.rs` — `World` struct, `WorldEvent` enum, `SpriteRenderRequest`.
`World` is always held in an `Arc<RwLock<World>>` in `GameData` and `PlayerData`.

## Subdirectories

### chunk_manager/
Owns the raw chunk data.
- `chunk_manager.rs` — `WorldChunkManager`: HashMap of `WorldChunkType` (Loaded/Lazy/Unloaded). Key = bit-packed `[i16;3]` via `chunk_cords_to_key`. Handles load/unload lifecycle and tik updates.
- `loaded_chunk.rs` — `LoadedWorldChunk`: 16×16×16 block array (`Box<[u16; 4096]>`), block entities HashMap, dynamic entities HashSet. Also declares `WorldChunkEvent` (sub-event pattern — called by WorldEvent handlers).
- `lazy_chunk.rs` / `unloaded_chunk.rs` — stub states for chunks not yet generated or evicted.

### chunk_tile_map_manager/
Owns the GPU mesh render layer for chunks. **DO NOT regress to per-tile texture draws.**
- `chunk_tile_set.rs` — `ChunkTileSet`: ray-casts one chunk on thread pool, bakes result into a GPU mesh. Has `lair_block_mods` for overlays, `dirty` flag.
- `chunk_tile_set_manager.rs` — `ChunkTileSetManager`: manages all tile sets. Key methods: `queue_dirty_chunk`, `queue_free_chunk`, `toggle_borders`, `get_obscuring`, `clean` (needs texture_manager + thread_pool, called from render_world), `render`.
- `chunk_render_data.rs` — `ChunkRenderData { scale, offset, center_world_cords, center_chunk_cords, chunk_view_range }`. Only `scale` and `offset` are used for sprite rendering.

### locations/
Geometry helpers — no game state.
- `world_area.rs` — `WorldArea` ([WorldPoint;2] bounding box). Key methods: `cords_in_area`, `fill_area`, `generate_border_mods`, `normalize_points`, `expand_to_fit_point`, `shrink_to_avoid_point`.
- `world_point.rs` — `WorldPoint { cords: [i32;3] }`.
- `world_area_side.rs` — enum for which face of an area to expand/shrink.

### world_gen/
Procedural generation — runs on chunk load.
- `world_gen.rs` / `WorldGenManager` — `generate_area(WorldArea) -> Vec<WorldEvent>`. Drives perlin + grass pass.
- `world_config.rs` — chunk rendering range and gen parameters.
- `terrain_gen/` — perlin noise + grass layer rules.

### world_task_manager/
`WorldTaskManager` — queues `ModBlockTask`s (world_cords + block_type) and executes them against the world + rendering system. Separate from WorldEvent — used for batched block modifications.

### world_data_manager/
Empty stub (octree planned but not implemented).

## Key patterns

**Reading world blocks:** `world.get_world_value([x,y,z]) -> u16` — returns block id.
**Writing blocks:** emit `WorldEvent::ModBlock(cords, block_type)` — handles dirty tile set automatically.
**Sprite rendering:** emit `WorldEvent::RenderSprite(SpriteRenderRequest { world_pos, texture })` — queued on `world.sprite_render_queue`, drained at end of `render_world` using that frame's `render_data`.
**Chunk lookup:** `World::world_cords_to_chunk_cords([i32;3]) -> [i16;3]` then `World::chunk_cords_to_key([i16;3]) -> u64`.

## Added 2026-05-18
- `SpriteRenderRequest` + `sprite_render_queue` on `World`
- `WorldEvent::RenderSprite` — queue sprite without borrowing world
- `WorldEvent::UnloadChunkAtCords` — debug chunk removal
- `WorldEvent::ToggleChunkBorders` — Selector overlay on chunk borders
- `generate_border_mods` on `WorldArea` — 8 corner overlays
- `ChunkTileSet` dirtying hooked to `ModBlock` / `ReplaceBlock` events

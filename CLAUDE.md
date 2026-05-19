# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Session Start

At the start of every session, greet Spencer by name. Read `memory/MEMORY.md` and the memory files most relevant to current work. Then briefly summarize: what was last worked on, what the current priorities are, and ask what he wants to tackle today. Keep it short — one sentence per point.

## Commands

```bash
cargo build          # compile
cargo run            # build and launch (2560×1440, non-resizable window)
cargo test           # run tests
cargo check          # fast type-check without linking
```

## What This Is

EdificeV3 is a Rust isometric voxel game built on `miniquad` (OpenGL). Players control a drone colony in an infinite procedurally-generated world. Drone AI was previously written in Lua (`mlua`) and is actively being replaced by a custom visual scripting system (`drone_script`).

## Architecture

### Entry point and per-frame loop

`src/main.rs` creates a `GameStage` that implements miniquad's `EventHandler`. Every frame, `GameData::render_camera` runs in this order:

1. `screen_manager.render_screen(...)` — renders the current frame
2. `tik_manager.new_update_tik_manager(...)` — advances simulation ticks
3. `event_manager.execute_*` — drains event queues (dispatch → input → player_data → world → render → widget)
4. `texture_manager.flush(...)` — uploads GPU draw batches

`GameData` is the root struct (`src/game_data/game_data.rs`) holding all subsystems.

### World (`src/game_data/world/`)

- Infinite voxel world stored as `HashMap<u64, WorldChunk>`
- Chunks are 16×16×16 (`CHUNK_VOLUME = 4096`), blocks stored as `Box<[u16; CHUNK_VOLUME]>`
- Chunk map key is bit-packed from `[i16; 3]` coords: `(z << 32) | (y << 16) | x`
- Chunks are lazy-generated on first access via `get_or_construct_chunk`
- `World` is wrapped in `Arc<RwLock<World>>` for threaded access
- World gen lives in `world_gen/` (perlin noise + grass pass); config in `WorldGenManager`

### Event System (`src/game_data/game_event_manager/`)

Events are the primary way to mutate shared state without `RefCell`. The four categories are `WidgetEvent`, `WorldEvent`, `RenderEvent`, and `PlayerDataEvent`. Systems push events during the frame; `EventManager` drains them at the end in a fixed order. Game objects are referenced by ID rather than direct pointers to enable this pattern.

### Rendering (`src/game_data/screen/`)

Isometric renderer built on ray casting:

- **`CastedTile`** — the smallest render unit: two triangles (left face + right face). Ray casting determines each triangle's texture.
- **Ray caster** (`renderer/ray_caster/`) — a ray stops at the first solid block and collects translucent textures along the way. Lives in a thread pool (`renderer/thread_manager/`).
- **`TileMap`** — a flattened 2D grid of `CastedTile`s for a world region. Managed by `TileMapManager`, which merges layers and marks regions dirty on block change.
- **Sprite occlusion** — after rendering a game-object sprite, the renderer re-draws all `CastedTriangle`s with greater depth on top, so terrain correctly occludes sprites.
- **Render cache** (`renderer/render_cache_manager/`) — cached rendered regions stored as canvas chunks to avoid re-raying unchanged areas.

`ScreenManager` owns the `Camera`, `CameraUIManager`, and `ScreenData`. It dispatches input to the widget tree and calls into the renderer.

### TextureManager (`src/game_data/texture_manager/`)

Single sprite atlas (`TextureAtlas`) containing all game art, subdivided into typed sheets: `block_sheet`, `block_triangle_sheet`, `shader_sheet`, `text_sheet`, `ui_sheet`. Rendering is GPU-batched via `RenderBatch` and flushed once per frame. `TextureCashe` stores pre-rendered region textures keyed by `CashedTextureID`.

### TikManager (`src/game_data/tik_manager/`)

Simulation tick system running at `tik_rate = 10,000 µs` (100 ticks/s). Each tick advances:

- `DroneManager` — moves drones, executes their scripts
- `LuaManager` — runs Lua-based drone AI (legacy; being phased out)
- `BlockUpdateManager` — plant growth and other block-level updates

### Drone Visual Scripting (`src/game_data/player_data/drone_script/`)

Replacement for Lua. A script is a sequence of `ScriptElement` nodes:

- `Var` — holds a typed value; `VarRef` links to an existing `Var`
- `Action` — a drone action with typed params and a return var
- `ControlFlow` — conditionals and loops
- `FunctionCall` / `Function` — user-defined functions with params and a return slot

Each `ScriptElement` implements `create_widget()` to produce its UI representation. Compilation pipeline is still in progress (see `plan.md` in that directory).

### Widget System (`src/game_data/screen/widget/`)

`Widget` trait requires `size()` (layout) and `render()`. All concrete types are wrapped in the `WidgetType` enum for homogeneous storage. Container widgets (`Panel`, `TabPanel`, `ScrollPanel`) hold `Vec<WidgetType>` children. Drone programming UI (`drone_programming/`) maps each `ScriptElement` variant to a corresponding slot widget (`VarSlot`, `ActionSlot`, `ControlFlowSlot`, etc.).

### PlayerData (`src/game_data/player_data/`)

Holds the player's runtime state: active drone scripts, drone inventory, settings, cursor position, progress manager, and the `WorldGenManager` instance used by the player's world config.

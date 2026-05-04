# game_data Directory Overview

The `game_data` module is the core of EdificeV3, organized into several subsystems.

---

## game_event_manager
Event bus for decoupled communication between all game systems. Every game state change routes through here. Sub-managers handle audio, render, widget, world, input, and player data events separately.

## player_data
All player-controlled entities and their state:
- **cursor** — Player selection/cursor tool
- **drone_script** — Script definitions for drone behavior (actions, control flow, functions, variables)
- **drones** — Drone entities, inventory, and tiered action system (primitive, getter, advanced)
- **locations** — Player-built structures and their blueprint system
- **nature_manager** — Non-player entities (currently: puff animals)
- **settings** — Game settings and configuration

## screen
The full UI and rendering pipeline:
- **camera_ui** — HUD overlays (drone mini-windows, tick speed display)
- **controls** — Input mappings for camera and drones
- **menu_constructors** — All menu layouts (main menu, play view, settings, world creation)
- **renderer** — Isometric ray-caster with shadow casting, multi-threaded block rendering, render cache
- **text** — Text rendering
- **ui_elements** — Basic reusable UI components (buttons, panels, text bars)
- **widget** — Advanced widget framework including the drone script editor UI and chunked world rendering

## tik_manager
The game loop. Drives all time-based simulation each tick: executes drone scripts, updates plant growth, and handles entity state changes.
- **block_updates** — Environmental simulation (plant growth)
- **drones** — Per-tick drone movement, physics, and script execution

## world
The 3D voxel world:
- **level_manager** — Predefined level templates (FlatField, Monoliths, Wall)
- **locations** — World-space coordinate types and area addressing
- **world_data_manager** — World data storage and an octree for spatial block lookup
- **world_gen** — Procedural terrain generation using Perlin noise
- **world_task_manager** — Async tasks for world loading and generation

## texture_manager
All 2D asset handling: texture atlasing, block sprites, character sprites, UI sprites, shader-based triangle rendering, and font sheet text rendering.

## types
Shared enum definitions used across the whole codebase — block types, item types, UI texture types, shader texture types, etc.

## tools
Coordinate conversion utilities (`cords_tool.rs`) for isometric projection math.

## logging_tool
Diagnostic logging utilities.

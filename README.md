# EdificeV3

## About
Edifice is a voxel game where you control a colony of drones to gather materials, craft tools to further your colony's abilities, and build whatever you like. Your colony exists in a world filled with life, some friendly, some not. You can either adapt your playstyle to avoid conflict or to embrace it.

## Architecture Overview

**Core Systems:**

- **World Rendering:**: Uses a custom isometric rendering engine. The isometric perspective allows for a number of
unique optimizations that are perfect for this type of game.

- **[Event System:](src/game_data/game_event_manager/event_manager.md)** The Event system is designed to allow diffrent for minipulation of the core game data in an orginized fassion.  

- **Widget System:** Uses a custom isometric rendering engine. The isometric perspective allows for a number of
unique optimizations that are perfect for this type of game.

- **World Storage:** 3D chunk based world storage where coordinates to these chunks are packed into a u64 key.

- **Tik Management:** 
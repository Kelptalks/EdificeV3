# EdificeV3

A voxel automation game where you program drones to build, mine, and automate

## About

The game is an open world voxel based game where you program drones to gather information about the world, and
have them perform actions to manipulate and traverse it. Drones have constraints like fuel and gravity that create
unique challenges the player must learn to automate. This automation allows for large scale resource gathering and
building that once programmed to execute can be built upon to accomplish greater and greater tasks.

## Architecture Overview

**Core Systems:**

- **World Rendering:** Uses a custom isometric rendering engine. The isometric perspective allows for a number of
unique optimizations that are perfect for this type of game.

- **World Storage:** 3D chunk based world storage where coordinates to these chunks are packed into a u64 key.

- **Tik Management:** 
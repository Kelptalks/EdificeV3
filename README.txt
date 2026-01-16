# EdificeV3

A voxel automation game where you program drones to build, mine, and automate

## About

The game is an open world voxel based game where you program drones to gather information about the world, and 
have them perform actions to minipulate and traverse it. Drones have constraints like fuel and gravity that create
uniqu challanges the player must learn to automate. This automation allows for large scale recorce gathering and 
building that once programmed to execute can be built apon to acomplish greater and greater tasks.

## Architecture Overview

**Core Systems:**

- **World Rendering:** Uses a Custom isometric rendering engine. The isometric perspective
- **World Storage:** Octree-based voxel system with bitpacked data
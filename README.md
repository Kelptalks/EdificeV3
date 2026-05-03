# EdificeV3

## About
Edifice is a voxel game where you control a colony of drones to gather materials, craft tools to further your colony's abilities, and build whatever you like. Your colony exists in a world filled with life, some friendly, some not. You can either adapt your playstyle to avoid conflict or to embrace it.

## Architecture Overview

**Core Systems:**

- **World Rendering:**: Uses a custom isometric rendering engine. The isometric perspective allows for a number of
unique optimizations that are perfect for this type of game.

- **Event System** The Event system is designed to allow diffrent for minipulation of the core game data in an orginized fassion. It allows for the use of id's in representing game objects to allow for mutiblity of objects with that id without resorting to Reffcells.


- **IsometricRenderer** The isometric perspective allows for a number of unique optimizations that are perfect for this type of game. The benifit makes is super effecent at cashing blocks, I can just save them to a texture and render huge quanitys, with an LOD system that is complicated by the infitnite world hight.

    ***World Rendering***

    - Casted Tile: casted tiles are the smallest rendering element the entire rendering system is built off. Casted consist of 2 Triangles left and right. What each of these triangles textures should be is determined through the ray casting algerithm.

    - Ray casting returns an array of all the textures intercepted by the ray. Rays stop only once they have hit a solid block. And return a Vec of all the translucent textures they hit on the way to that block.  

    ***Render Cashing***

    - Tile Map


- **Widget System:** 

- **World Storage:** 3D chunk based world storage where coordinates to these chunks are packed into a u64 key.

- **Game Object Management:** 
    Game Objects are entitys that exist in the world. They are represented by ID's that can be accsessed to get a copy or create events to minipulate them.



    - Location Mangment: objects id's are stored in world chunks to allow for accsessing them spacialy and saving to disk. Chunks store a list of id's that they contain.  
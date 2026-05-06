# EdificeV3

## About
Edifice is a voxel game where you control a colony of drones to gather materials, craft tools to further your colony's abilities, and build whatever you like. Your colony exists in a world filled with life, some friendly, some not. You can either adapt your playstyle to avoid conflict or to embrace it. The engine is a highly optimized isometric voxel renderer so that it can handle simulating the complex world.

## Architecture Overview

**Core Systems:**

- **World:**: the world is stored in a hashmap of chunks that store block data in a linear array indexed by a bitpacked u64 key
    ***World Chunk Contents***
        - Blocks: Blocks are what the world is made of blocks are stored as u32 ids
        - GameObjectIds: Chunks contain a list of all the game objects currently within them. 
        - TileMapId: tile map representing the area of the chunk.
    

- **Event System** The Event system is designed to allow diffrent for minipulation of the core game data in an orginized fassion. It allows for the use of id's in representing game objects to allow for mutiblity of objects without resorting to Reffcells. There are 2 diffrent diffrent times where events are executed.
    - Game Object Id's
    

- **IsometricRenderer** The isometric perspective allows for a number of unique optimizations that are perfect for this type of game. The benifit makes is super effecent at cashing blocks, I can just save them to a texture and render huge quanitys, with an LOD system that is complicated by the infitnite world hight.

    ***World Rendering***

    - Casted Tile: casted tiles are the smallest rendering element the entire rendering system is built off. Casted consist of 2 Triangles left and right. What each of these triangles textures should be is determined through the ray casting algerithm.

    - Ray casting returns an array of all the textures intercepted by the ray. Rays stop only once they have hit a solid block. And return a Vec of all the translucent textures they hit on the way to that block.  

    ***Render Cashing***
    - Tile Map: A tile map is a flattened collection of casted tiles.

    - Tile Manager: Manages the flatting of a collection of tile maps. The flattened map contains only the casted tile that hit a solid block of the highst depth. The flattend map is updated when a tile map lair is dirtyed. 
    

    ***Game Object Rendering***
    
    - Game Objects have sprites that are retreaved based on the cameras current view direction. Sprites 3D world pos are converted to flattened tile cords. After the sprite is rendered we re render all casted triangles that have a lower depth then the sprite over top of the sprite. This allows sprites to be obscured by objects.  


- **Widget System:** 
    - 

- **Game Object Management:** 
    - Game Objects are entitys that exist in the world. They are represented by ID's that can be accsessed to get a clone or create events to minipulate them using the objects event sceduler. 

    - Location Mangment: objects id's are stored in world chunks to allow for accsessing them spacialy and saving to disk. Chunks store a list of id's that they contain.  



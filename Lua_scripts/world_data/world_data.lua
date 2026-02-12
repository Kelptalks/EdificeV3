local Quarry = require("world_data.structures.quarry")

--[[
################
## World Data ##
################

## purpose 
This is the object that stores all the information about the world that drones
have collected. It also stores locations of structures.

## World information

## Todo
 - Store explored nodes in world data


--]]

---@class WorldData
WorldData = {
    quarry = Quarry.new({35, 35}, 15)
}

return WorldData
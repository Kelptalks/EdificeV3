
local BlockType = require("rust_wrapper_functions.block_types")
local M = {}  -- Module table

-- Drone Getter Functions
function M.get_all_drone_ids() 
    return rust_get_all_drone_ids()
end

--- scan a block relative to drones position
--- @param drone_id integer
--- @param x integer
--- @param y integer
--- @param z integer
--- @return integer id of block
function M.scan_block(drone_id, x, y, z) 
    return rust_scan_block(drone_id, x, y, z)
end

function M.is_busy(drone_id)
    return rust_is_busy(drone_id)
end

function M.has_item_amount(drone_id, item) 
    return rust_get_item_quantity(drone_id, item)
end

--- Get the cords of the drone in the world
--- @param drone_id integer
--- @return table 3D cords of drones position
function M.get_drone_cords(drone_id)
    return rust_get_drone_cords(drone_id)
end

function M.get_vision_range(drone_id)
    return rust_get_vision_range(drone_id)
end

-- Drone Action Functions
function M.move(drone_id, x, y, z)
    return rust_move(drone_id, x, y, z)
end

--- Mine a block relative to the drones current position
--- @param drone_id integer
--- @param x integer
--- @param y integer
--- @param z integer
function M.mine_block(drone_id, x, y, z)
    rust_mine_block(drone_id, x, y, z)
end

function M.place_block(drone_id, x, y, z, blocktype)
    rust_place_block(drone_id, x, y, z, blocktype)
end

function M.craft_item(drone_id, item) 
    rust_craft_item(drone_id, item)
end

return M
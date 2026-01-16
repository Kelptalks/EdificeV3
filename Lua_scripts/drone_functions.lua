
local BlockType = require("block_types")
local M = {}  -- Module table

-- Drone Getter Functions
function M.get_all_drone_ids() 
    return rust_get_all_drone_ids()
end

function M.scan_block(drone_id, x, y, z) 
    return rust_scan_block(drone_id, x, y, z)
end

function M.is_busy(drone_id)
    return rust_is_busy(drone_id)
end

-- Drone Action Functions
function M.move(drone_id, x, y, z)
    rust_move(drone_id, x, y, z)
end

function M.mine_block(drone_id, x, y, z)
    rust_mine_block(drone_id, x, y, z)
end

function M.place_block(drone_id, x, y, z, blocktype)
    rust_place_block(drone_id, x, y, z, blocktype)
end

function M.has_item_amount(drone_id, amount) 
    return rust_get_item_quantity(drone_id, amount)
end

function M.craft_item(drone_id, item) 
    rust_craft_item(drone_id, item)
end

return M
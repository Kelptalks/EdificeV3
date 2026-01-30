local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")

local GatherGoal = {}  -- Module table
GatherGoal.__index = GatherGoal  -- Add this!

function GatherGoal.new()
    local self = setmetatable({}, GatherGoal)
    self.complete = false
    self.blocks_to_gather = {}
    return self
end

function GatherGoal:is_complete()
    return self.complete
end

function GatherGoal:add_block_to_gather(block_id)
    table.insert(self.blocks_to_gather, block_id)
end

function GatherGoal:tik(drone_id)
    -- Scan area in mine range
    for z = -1, 1 do
        for y = -1, 1 do
            for x = -1, 1 do
                local block_type_scanned = DroneFunctions.scan_block(drone_id, x, y, z)
                -- Check if block matches one of the types we want to mine
                for i, block_type_to_mine in ipairs(self.blocks_to_gather) do
                    if block_type_scanned == block_type_to_mine then
                        DroneFunctions.mine_block(drone_id, x, y, z)
                        return true;
                    end
                end
            end
        end
    end
    return false;
end


return GatherGoal
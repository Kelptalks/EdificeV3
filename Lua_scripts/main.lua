local DroneFunctions = require("drone_functions")
local BlockType = require("block_types")

function MoveFowardOrUp(drone_id, x, y) 
    -- Check block foward
    local BlockTypeInFront = DroneFunctions.scan_block(drone_id, x, y, 0)
    if not BlockType.is_solid(BlockTypeInFront) then 
        DroneFunctions.move(drone_id, x, y, 0)
        return true
    end

    -- Check block foward and up
    local BlockTypeInFrontUp = DroneFunctions.scan_block(drone_id, x, y, 1)
    if not BlockType.is_solid(BlockTypeInFrontUp) then
        DroneFunctions.move(drone_id, x, y, 1)
        return true
    end
    return false
end

function tik()
    local x, y, z = 0, 0, -1
    
    local drone_ids = DroneFunctions.get_all_drone_ids()

    -- Loops through all the drones
    for i, drone_id in ipairs(drone_ids) do
        local my_drone_id = drone_id

        if not DroneFunctions.is_busy(my_drone_id) then
            if not MoveFowardOrUp(drone_id, 1, 0) then
                if not MoveFowardOrUp(drone_id, 0, 1) then
                    if not MoveFowardOrUp(drone_id, 0, -1) then
                
                    end
                end
            end
        end
        
    end

end
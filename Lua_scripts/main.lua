local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockType = require("rust_wrapper_functions.block_types")


local drone_manager = require("drone_manager.drone_manager")

function tik()
    local x, y, z = 0, 0, -1
    
    local drone_ids = DroneFunctions.get_all_drone_ids()

    -- Loops through all the drones
    for i, drone_id in ipairs(drone_ids) do
        local drone = drone_manager:get_drone(drone_id)
        drone:tik()
    end

end
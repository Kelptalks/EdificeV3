local Drone = require("drone_manager.drone")

local drone_manager = {
    drones = {}
}

function drone_manager:get_drone(drone_id)
    if not self.drones[drone_id] then
        self.drones[drone_id] = Drone.new(drone_id)
        print("Created new drone with id: " .. drone_id)
    end
    
    return self.drones[drone_id]
end

return drone_manager

local GoalManager = require("drone_goals.goal_manager")
local DroneFunctions = require("rust_wrapper_functions.drone_functions")

local Drone = {}
Drone.__index = Drone

function Drone.new(id)
    local self = setmetatable({}, Drone)
    self.id = id
    self.goal_manager = GoalManager:new()

    self.goal_manager:add_move_goal(50, 50, 0)
    return self
end

function Drone:tik()
    if DroneFunctions.is_busy(self.id) then
        return
    end

    -- Drone update logic
    self.goal_manager:execute(self.id)
end

function Drone:add_move_goal(x, y, z)
    self.goal_manager:add_move_goal(x, y, z)
end

return Drone
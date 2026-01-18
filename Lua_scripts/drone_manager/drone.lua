
local GoalManager = require("drone_goals.goal_manager")
local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local ScavengeGoal = require("drone_goals.resource_gathering.scavenge_goal")
local Drone = {}
Drone.__index = Drone

function Drone.new(id)
    local self = setmetatable({}, Drone)
    self.id = id
    self.goal_manager = GoalManager:new()

    local scavenge_goal = ScavengeGoal.new(20, 20, 50, 50)
    self.goal_manager:incert_goal(scavenge_goal, 1)
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
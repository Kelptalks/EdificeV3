
local GoalManager = require("drone_goals.goal_manager")
local DroneFunctions = require("rust_wrapper_functions.drone_functions")

-- Goals
local TreeChoppingGoal = require("drone_goals.resource_gathering.tree_chopping_goal")
local MoveGoal = require("Lua_scripts.drone_goals.basic.move_goal")


local Drone = {}
Drone.__index = Drone


--- comment
--- @param id integer
--- @return table
function Drone.new(id)
    local self = setmetatable({}, Drone)
    self.id = id
    self.goal_manager = GoalManager:new()


    -- local move_goal = MoveGoal.new(-10, 100, 0)
    -- self.goal_manager:incert_goal(move_goal, 1)


    local tree_chopping_goal = TreeChoppingGoal.new(id, -95, -95, 95, 95)
    self.goal_manager:incert_goal(tree_chopping_goal, 1)


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
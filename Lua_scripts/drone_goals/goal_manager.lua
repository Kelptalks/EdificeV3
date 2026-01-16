local DroneFunctions = require("rust_wrapper_functions.block_types")
local MoveGoal = require("Lua_scripts.drone_goals.basic_goals.move_goal")

local GoalManager = {}  -- Module table
GoalManager.__index = GoalManager  -- Add this!

function GoalManager.new()
    local self = setmetatable({}, GoalManager)
    self.goals = {}

    return self
end

function GoalManager:execute(drone_id)
    local first_goal = self.goals[1]
    if first_goal ~= nil then
        first_goal:tik(drone_id, self)
        -- Remove goal if completed
        if first_goal:is_complete() then
            table.remove(self.goals, 1)
        end
    end

end

function GoalManager:add_move_goal(x, y, z)
    local move_goal = MoveGoal.new(x, y, z)  -- Assuming MoveGoal is in BasicGoals
    table.insert(self.goals, 1, move_goal)  -- Insert at position 1, shifts others right
end

return GoalManager
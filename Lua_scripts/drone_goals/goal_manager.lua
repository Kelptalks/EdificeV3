local DroneFunctions = require("rust_wrapper_functions.drone_functions")

local GoalManager = {}  -- Module table
GoalManager.__index = GoalManager  -- Add this!

function GoalManager.new()
    local self = setmetatable({}, GoalManager)
    self.goals = {}

    return self
end

function GoalManager:execute(drone_id)
    -- Loop through goals and break loop when drone is busy
    for i, goal in ipairs(self.goals) do
        goal:tik(drone_id)
        -- Remove goal if completed
        if goal:is_complete() then
            table.remove(self.goals, i)
        end

        if DroneFunctions.is_busy(drone_id) then
            return true
        end
    end
    return false
end

function GoalManager:has_goal()
    return #self.goals > 0
end

function GoalManager:get_goal_count()
    return #self.goals
end

-- Insert a goal at index
function GoalManager:incert_goal(goal, index)
    table.insert(self.goals, index, goal)
end

-- add goal at index
function GoalManager:add_goal(goal)
    table.insert(self.goals, goal)
end


return GoalManager
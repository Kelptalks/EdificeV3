local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")

local GoalManager = require("drone_goals.goal_manager")
local MoveGoal = require("drone_goals.basic.move_goal")
local GatherGoal = require("drone_goals.basic.gather_goal")

local ScavengeGoal = {}  -- Module table
ScavengeGoal.__index = ScavengeGoal  -- Add this!

function ScavengeGoal.new(x_start_cor, y_start_cor, x_end_cor, y_end_cor)
    local self = setmetatable({}, ScavengeGoal)
    self.complete = false
    self.goal_manager = GoalManager.new()

    -- Area to clear
    self.start_cords = {x_start_cor, y_start_cor}
    self.end_cords = {x_end_cor, y_end_cor}

    -- Steps
    self.started = false
    self.started_gathering = false

    return self
end

function ScavengeGoal:is_complete()
    return self.complete
end


function ScavengeGoal:tik(drone_id)
    -- Execute goal manager if it has goals
    if self.goal_manager:has_goal() then
        self.goal_manager:execute(drone_id)
        return;
    end
    
    print("Tik")
    -- Walk to starting location
    if not self.started then
        print("Planed walk to start")
        local move_goal = MoveGoal.new(self.start_cords[1], self.start_cords[2], 0)
        self.goal_manager:incert_goal(move_goal, 1)
        self.started = true
        return
    -- Add the gather goal with block types
    elseif not self.started_gathering then
        print("started gathering")
        local gather_goal = GatherGoal.new()
        gather_goal:add_block_to_gather(BlockTypes.white_flowers)
        gather_goal:add_block_to_gather(BlockTypes.yellow_flowers)
        gather_goal:add_block_to_gather(BlockTypes.mushroom)
        gather_goal:add_block_to_gather(BlockTypes.flungle)
        gather_goal:add_block_to_gather(BlockTypes.rock)
        gather_goal:add_block_to_gather(BlockTypes.log)
        self.goal_manager:incert_goal(gather_goal, 1)
        self.started_gathering = true
        return
    end



end

return ScavengeGoal
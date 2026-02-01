-- Drone and world
local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")

-- Goals
local GoalManager = require("drone_goals.goal_manager")
local MoveGoal = require("drone_goals.basic.move_goal")
local GatherGoal = require("drone_goals.basic.gather_goal")

local ScavengeGoal = {}
ScavengeGoal.__index = ScavengeGoal

function ScavengeGoal.new(x_start_cor, y_start_cor, x_end_cor, y_end_cor)
    local self = setmetatable({}, ScavengeGoal)
    self.complete = false
    
    -- Setup Cords
    self.start_cords = {x_start_cor, y_start_cor}
    self.end_cords = {x_end_cor, y_end_cor}

    -- Setup rows
    self.rows = math.ceil(math.abs(x_start_cor - x_end_cor) / 3)
    self.current_row = 0;


    -- Setup move goal to get to starting cords
    self.current_move_goal = MoveGoal.new(self.start_cords[1], self.start_cords[2], 0)
    
    -- Setup Gathering goal 
    local gather_goal = GatherGoal.new()
    gather_goal:add_block_to_gather(BlockTypes.white_flowers)
    gather_goal:add_block_to_gather(BlockTypes.yellow_flowers)
    gather_goal:add_block_to_gather(BlockTypes.mushroom)
    gather_goal:add_block_to_gather(BlockTypes.flungle)
    gather_goal:add_block_to_gather(BlockTypes.rock)
    gather_goal:add_block_to_gather(BlockTypes.log)
    self.gather_goal = gather_goal

    return self
end

function ScavengeGoal:is_complete()
    return self.complete
end


function ScavengeGoal:tik(drone_id)
    
    -- If current goal is not complete
    if not self.current_move_goal:is_complete() then
        -- attempt to gather item
        if self.gather_goal:tik(drone_id) then
            return;
        -- If no item was gathered move
        else
            self.current_move_goal:tik(drone_id)
        end

    -- If the a new move goal needs to be created
    else
        if self.current_row < self.rows then
            if self.current_row % 2 == 1 then
                self.current_move_goal = MoveGoal.new(self.start_cords[1] + (3 * self.current_row), self.start_cords[2], 0)
            else
                self.current_move_goal = MoveGoal.new(self.start_cords[1] + (3 * self.current_row), self.end_cords[2], 0)
            end
            self.current_row = self.current_row + 1
        else
            self.complete = true
        end
    end

end




return ScavengeGoal
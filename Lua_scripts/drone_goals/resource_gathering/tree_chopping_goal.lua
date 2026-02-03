-- Drone and world
local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")

-- Goals
local MoveGoal = require("drone_goals.basic.move_goal")

local ChopTreesInAreaGoal = {}
ChopTreesInAreaGoal.__index = ChopTreesInAreaGoal


--- comment
--- @param drone_id integer
--- @param x_start_cor integer
--- @param y_start_cor integer
--- @param x_end_cor integer
--- @param y_end_cor integer
--- @return table
function ChopTreesInAreaGoal.new(drone_id, x_start_cor, y_start_cor, x_end_cor, y_end_cor)
    local self = setmetatable({}, ChopTreesInAreaGoal)

    -- Get the row width based off the drones vission to allow for effective scanning
    self.row_width = (DroneFunctions.get_vision_range(drone_id) * 2)

    print("Created Tree Chopping Goal")
    -- Goal Controls
    self.complete = false

    -- Setup Cords
    self.start_cords = {x_start_cor, y_start_cor}
    self.end_cords = {x_end_cor, y_end_cor}
    self.moved_to_start = false;


    -- Scanning area setup
    self.rows = math.ceil(math.abs(x_start_cor - x_end_cor) / self.row_width)
    self.current_row = 0;
    self.current_move_goal = MoveGoal.new(x_start_cor, y_start_cor, 0)
    


    -- Tree 
    self.found_tree = false
    self.move_to_tree_goal = nil

    -- Tree Chopping
    self.at_tree = false
    self.side_of_tree = {0, 0}

    return self
end

--- comment
--- @return boolean
function ChopTreesInAreaGoal:is_complete()
    return self.complete
end

--- func desc
---@param drone_id integer
function ChopTreesInAreaGoal:tik(drone_id)
    local drone_cords = DroneFunctions.get_drone_cords(drone_id)

    -- Chop tree
    if self.at_tree then
        -- Piller if not at the top of the tree
        local block = DroneFunctions.scan_block(drone_id, self.side_of_tree[1], self.side_of_tree[2], 1)
        if block == BlockTypes.BrownTrunk then
            -- Place block on itself to use pillering tech
            DroneFunctions.place_block(drone_id, 0, 0, 0, BlockTypes.Scaffolding)
            return;

        -- If the drone is at the top of the tree
        else
            local block_to_mine = DroneFunctions.scan_block(drone_id, self.side_of_tree[1], self.side_of_tree[2], 0)
            
            -- Mine if log is next to drone
            if block_to_mine == BlockTypes.BrownTrunk then
                DroneFunctions.mine_block(drone_id, self.side_of_tree[1], self.side_of_tree[2], 0)
                return false;

            -- Mine down if there is more wood bellow
            elseif DroneFunctions.scan_block(drone_id, self.side_of_tree[1], self.side_of_tree[2], -1) == BlockTypes.BrownTrunk then
                DroneFunctions.mine_block(drone_id, 0, 0, -1)
                return false;

            -- Tree has been completly chopped
            else
                -- Reset variables after tree has been cleared
                self.at_tree = false;
                self.found_tree = false;

                return false;
            end


        end




    -- Move towards tree
    elseif self.found_tree then
        if self.move_to_tree_goal:is_complete() then
            self.move_to_tree_goal = nil

            -- Setup next goal
            self.at_tree = true
            -- Figure out the direction of the tree
            for y = -1, 1 do
                for x = -1, 1 do
                    for z = 0, 1 do
                        local block_type_scanned = DroneFunctions.scan_block(drone_id, x, y, z)
                        if block_type_scanned == BlockTypes.BrownTrunk then
                            self.side_of_tree = {x, y}
                        end
                    end
                end
            end
            return false
        else
            self.move_to_tree_goal:tik(drone_id)
            return false
        end
    -- Scan for trees
    else
        -- Loop through scan range
        local vission_range = DroneFunctions.get_vision_range(drone_id)
        -- Scan area in mine range
        for z = -vission_range, vission_range do
            for y = -vission_range, vission_range do
                for x = -vission_range, vission_range do
                    -- If the block scanned is a log plan a walk twards it
                    local block_type_scanned = DroneFunctions.scan_block(drone_id, x, y, z)
                    if block_type_scanned == BlockTypes.BrownTrunk then
                        -- Setup tree location and goal to navigate to tree
                        self.move_to_tree_goal = MoveGoal.new(drone_cords[1] + x, drone_cords[2] + y, drone_cords[3] + z)
                        self.found_tree = true

                        return false;
                    end
                end
            end
        end

        -- Continue moving if nothing could be found
        if not self.current_move_goal:is_complete() then
            self.current_move_goal:tik(drone_id)
            return false
        else
            -- Plan next row movment based off current row
            if self.current_row < self.rows then
            if self.current_row % 2 == 1 then
                self.current_move_goal = MoveGoal.new(self.start_cords[1] + (self.row_width * self.current_row), self.start_cords[2], 0)
            else
                self.current_move_goal = MoveGoal.new(self.start_cords[1] + (self.row_width * self.current_row), self.end_cords[2], 0)
            end
            
            self.current_row = self.current_row + 1
            else
                self.complete = true
            end
        end
    end
end




return ChopTreesInAreaGoal
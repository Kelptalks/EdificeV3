local WorldData = require("world_data.world_data")

-- Rust wrappers
local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")
local DroneItems = require("rust_wrapper_functions.drone_items")

local Quarry = require("world_data.structures.quarry")
local MoveGoal = require("drone_goals.basic.move_goal")

local QuarryGoal = {}
QuarryGoal.__index = QuarryGoal

local row_width = 2

--- mine at the quarry in world data
--- @param goal_stone_amount integer amount of stone to gather
--- @return table new quarrying goal
function QuarryGoal.new(goal_stone_amount)
    local self = setmetatable({}, QuarryGoal)

    self.complete = false;

    local quarry = WorldData.quarry;
    local quarry_location = quarry.start_cords;
    local quarry_scale = quarry.scale;

    print("Quarry location ".. quarry_location[1].. ","..quarry_location[2])

    self.current_move_goal = MoveGoal.new(quarry_location[1], quarry_location[2], 0)
    
    -- Goals
    self.goal_stone_amount = goal_stone_amount
    self.exit_quarry = false

    -- Setup rows
    self.rows = math.ceil(math.abs(quarry_scale / row_width))
    self.current_row = 0
    self.highest_block_found_z = -999999

    return self
end

---
---@return boolean
function QuarryGoal:is_complete()
    return self.complete
end


--- tik the quarry goal
--- @param drone_id integer
function QuarryGoal:tik(drone_id)
    -- Setup data
    -- Quarry data
    local quarry = WorldData.quarry
    local quarry_location = quarry.start_cords
    local quarry_scale = quarry.scale
    local quarry_depth = quarry.current_depth

    -- Drone Data
    local drone_cords = DroneFunctions.get_drone_cords(drone_id)


    if self.exit_quarry then
        if self.current_move_goal:is_complete() then        
            local x_corner_distance = drone_cords[1] - quarry_location[1];
            local y_corner_distance = drone_cords[2] - quarry_location[2];

            if x_corner_distance > 0 then
                print("Correcting X")
                DroneFunctions.move(drone_id, -1, 0, 0)
                return;
            elseif y_corner_distance > 0 then
                print("Correcting Y: ".. y_corner_distance)
                DroneFunctions.move(drone_id, 0, -1, 0)
                return;
            else
                if DroneFunctions.scan_block(drone_id, -1, 0, 0) == BlockTypes.Air then
                    self.complete = true;
                else
                    DroneFunctions.place_block(drone_id, 0, 0, 0, BlockTypes.Scaffolding)
                    return;
                end
            end
            



            return;
        else
            self.current_move_goal:tik(drone_id);
            return;
        end
    end

    -- Check if acheaved goal amount of stone
    if DroneFunctions.has_item_amount(drone_id, DroneItems.Stone) >= self.goal_stone_amount then
        print("completed goal")
        self.exit_quarry = true
        -- Move back to starting location
        self.current_move_goal = MoveGoal.new(quarry_location[1], quarry_location[2], 0);
        return;
    end

    -- attempt to mine all blocks in quarry in range of drone_goals
    -- Scan all blocks in mine range
    for x = -1, 1 do
        for y = -1, 1 do
            for z = -1, 1 do
                -- Skip if on drone
                if x + y + z ~= 0 then
                    -- get block data
                    local block_type = DroneFunctions.scan_block(drone_id, x, y, z)
                    local world_cords_of_block = {
                        drone_cords[1] + x,
                        drone_cords[2] + y,
                        drone_cords[3] + z,
                    }

                    -- If block is not air
                    if block_type ~= BlockTypes.Air then
                        -- if block is in the area of the quarry
                        if quarry:block_in_quarry_area(world_cords_of_block) then
                            if world_cords_of_block[3] > self.highest_block_found_z then
                                self.highest_block_found_z = world_cords_of_block[3]
                            end
                            if quarry:block_in_quarry_depth(world_cords_of_block) then
                                DroneFunctions.mine_block(drone_id, x, y, z)
                                return;
                            end
                        end
                    end
                end
            end
        end
    end

    -- Move to quarry entrence
    if not self.current_move_goal:is_complete() then
        self.current_move_goal:tik(drone_id)
        return
    else
        -- If the a new move goal needs to be created
        if self.current_row < self.rows then
            if self.current_row % 2 == 1 then
                self.current_move_goal = MoveGoal.new(
                    quarry_location[1] + (row_width * self.current_row), quarry_location[2], 0
                )
            else
                self.current_move_goal = MoveGoal.new(
                    quarry_location[1] + (row_width * self.current_row), quarry_location[2] + quarry_scale, 0
                )
            end
            self.current_row = self.current_row + 1
        else
            -- Update quarry depth
            -- Only update if we found blocks (not still at initial value)
            if self.highest_block_found_z > -999999 then
                quarry.current_depth = self.highest_block_found_z
            else

            end
            
            -- Reset movment
            self.highest_block_found_z = -999999
            self.current_move_goal = MoveGoal.new(quarry_location[1], quarry_location[2], 0)
            self.current_row = 0;
        end
    end
    



end

return QuarryGoal
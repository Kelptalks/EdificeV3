local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")

local MoveGoal = {}  -- Module table
MoveGoal.__index = MoveGoal  -- Add this!

function MoveGoal.new(x, y, z)
    local self = setmetatable({}, MoveGoal)
    self.goal_cords = {x, y, z}
    self.complete = false
    return self
end


local function move(drone_id, x_direction, y_direction)
    local z_direction = -1;
    local block_in_front_down = DroneFunctions.scan_block(drone_id, x_direction, y_direction, z_direction)
    if not BlockTypes.is_solid(block_in_front_down) then 
        DroneFunctions.move(drone_id, x_direction, y_direction, z_direction)
        return true;
    end

    local z_direction = 0;
    local block_in_front_down = DroneFunctions.scan_block(drone_id, x_direction, y_direction, z_direction)
    if not BlockTypes.is_solid(block_in_front_down) then 
        DroneFunctions.move(drone_id, x_direction, y_direction, z_direction)
        return true;
    end

    local z_direction = 1;
    local block_in_front_down = DroneFunctions.scan_block(drone_id, x_direction, y_direction, z_direction)
    if not BlockTypes.is_solid(block_in_front_down) then 
        DroneFunctions.move(drone_id, x_direction, y_direction, z_direction)
        return true;
    end

    return false;
end



function MoveGoal:is_complete()
    return self.complete
end

function MoveGoal:tik(drone_id, goal_manager)
    local current_cords = DroneFunctions.get_drone_cords(drone_id);

    local x_distance = (current_cords[1] - self.goal_cords[1])
    local y_distance = (current_cords[2] - self.goal_cords[2])

    local x_distance_abs = math.abs(x_distance)
    local y_distance_abs = math.abs(y_distance)

    -- If goal has been complete
    if x_distance == 0 and y_distance == 0 then
        self.complete = true
        return
    end

    local x_direction = -(x_distance / math.abs(x_distance));
    local y_direction = -(y_distance / math.abs(y_distance));



    -- Pathing algorithm
    -- I need to a locale map that saves node data
    --  - (distance from drone + distance from goal).
    --  - Connected node
    --  - node cords
    -- 
    --  Create nodes around lowest h value first, and incert into a list based off lowest h value
    --  










    -- Try moving in x direction
    if x_distance_abs > 0 then
        -- Try and move in x direction
        if move(drone_id, x_direction, 0) then
            return
        end

        -- try and move to sides if failed
        if move(drone_id, 0, 1) then
            return
        end

        if move(drone_id, 0, -1) then
            return
        end
    end

    -- try moving in y direction
    if y_distance_abs > 0 then
        -- Try and move in y direction
        if move(drone_id, 0, y_direction) then
            return
        end

        -- try and move to sides if failed
        if move(drone_id, 1, y_direction) then
            return
        end

        if move(drone_id, -1, y_direction) then
            return
        end
    end



end

return MoveGoal
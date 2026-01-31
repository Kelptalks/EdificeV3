local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")

local MoveGoal = {}  -- Module table
MoveGoal.__index = MoveGoal  -- Add this!

function MoveGoal.new(x, y, z)
    local self = setmetatable({}, MoveGoal)
    self.goal_cords = {x, y, z}
    self.complete = false
    self.nodes = {} -- Nodes are all walkable locations discovered by the drone
    self.current_path = {} -- The current path the drone is attempting to execute

    return self
end

--[[
~~~~~~~~~~~~~
~~ Movment ~~
~~~~~~~~~~~~~
--]]
local function is_block_cords_safe(drone_id, x, y, z)
    local block_type = DroneFunctions.scan_block(drone_id, x, y, z)
    -- Can't walk inside solid block
    if BlockTypes.is_solid(block_type) then
        return false
    else
        -- Make sure block below is walkable and not itself
        block_type = DroneFunctions.scan_block(drone_id, x, y, z -1)
        if BlockTypes.is_solid(block_type) then
            return true
        else 
            return false
        end
    end
end

-- Add a node to the nodes table
function MoveGoal:reconstruct_path(came_from, current_key, start_key)
    local path = {}
    -- Walk backwards from current to start
    while current_key ~= start_key do
        table.insert(path, 1, current_key)  -- Insert at beginning
        current_key = came_from[current_key]
        
        if current_key == nil then
            print("ERROR: Path reconstruction failed")
            return nil
        end
    end
    
    return path  -- Returns array of keys from start to goal
end

--[[
~~~~~~~~~~~~~~~~~~~
~~ Node controls ~~
~~~~~~~~~~~~~~~~~~~
--]]

-- Add a node to the nodes table
function MoveGoal:add_node(node_cords, connected_node_cords, drone_cords)
    local x_distance_from_drone = math.abs(node_cords[1] - drone_cords[1])
    local y_distance_from_drone = math.abs(node_cords[2] - drone_cords[2])
    local z_distance_from_drone = math.abs(node_cords[3] - drone_cords[3])

    local distance_from_drone = x_distance_from_drone + y_distance_from_drone + z_distance_from_drone

    local x_distance_from_goal = math.abs(node_cords[1] - self.goal_cords[1])
    local y_distance_from_goal = math.abs(node_cords[2] - self.goal_cords[2])
    local z_distance_from_goal = math.abs(node_cords[3] - self.goal_cords[3])
    local distance_from_goal = x_distance_from_goal + y_distance_from_goal + z_distance_from_goal
    
    self.nodes[node_cords[1].."_"..node_cords[2].."_"..node_cords[3]] = {
        ["Cords"] = node_cords,
        ["came_from"] = connected_node_cords,
        ["distance_from_drone"] = distance_from_drone,
        ["distance_from_goal"] = distance_from_goal,
    }
end

-- Update all the nodes for walkability in the drones vision range
function MoveGoal:discover_visible_nodes(drone_id)
    local vision_radius = DroneFunctions.get_vision_range(drone_id)
    local drone_cords = DroneFunctions.get_drone_cords(drone_id)

    -- Scan all positions within vision range
    for x = -vision_radius, vision_radius do
        for y = -vision_radius, vision_radius do
            for z = -vision_radius, vision_radius do
                local node_cords = {
                    drone_cords[1] + x,
                    drone_cords[2] + y,
                    drone_cords[3] + z
                }
                local key = node_cords[1].."_"..node_cords[2].."_"..node_cords[3]

                if is_block_cords_safe(drone_id, x, y, z) then
                    -- add if we haven't seen it
                    if not self.nodes[key] then
                        self:add_node(node_cords, nil, drone_cords)
                    end
                else 
                    -- Remove block that has changed in walkabilty
                    if self.nodes[key] then
                        self.nodes[key] = nil
                    end
                end
            end
        end
    end
end

-- Get the lowest h value node
function MoveGoal:get_lowest_h_value_node_key(open_set)
    local lowest_h_value = math.huge
    local lowest_node_key = nil
    for i, node_key in pairs(open_set) do
        local node = self.nodes[node_key] -- get the node from my map using key
        -- If the node is ok
        if node then
            local node_h_value = node["distance_from_drone"] + node["distance_from_goal"]
            if node_h_value < lowest_h_value then
                lowest_h_value = node_h_value
                lowest_node_key = node_key
            end 
        end
    end
    return lowest_node_key
end

-- Get the naboring nodes with a node cords | needed for when node does not exist
function MoveGoal:get_neighbors_with_node_cords(node_cords)
    local neighbors = {}
    local cords = node_cords
    
    -- Check 4 directions (or 8 if you want diagonals)
    local directions = {
        {1, 0, 0},   -- right
        {-1, 0, 0},  -- left
        {0, 1, 0},   -- forward
        {0, -1, 0}   -- back
    }
    
    for _, dir in ipairs(directions) do
        -- Loop through hights
        for z = -1, 1 do
            local neighbor_key = (cords[1] + dir[1]).."_"..(cords[2] + dir[2]).."_"..(cords[3] + z)
        
            -- Only include if we've discovered it (it's in self.nodes)
            if self.nodes[neighbor_key] then
                table.insert(neighbors, neighbor_key)
            end
        end
    end
    
    return neighbors
end

-- Get the naboring nodes with a node key
function MoveGoal:get_neighbors_with_key(node_key)
    local node = self.nodes[node_key]
    local neighbors = {}
    local cords = node["Cords"]
    
    -- Check 4 directions (or 8 if you want diagonals)
    local directions = {
        {1, 0, 0},   -- right
        {-1, 0, 0},  -- left
        {0, 1, 0},   -- forward
        {0, -1, 0}   -- back
    }
    
    for _, dir in ipairs(directions) do
        -- Loop through hights
        for z = -1, 1 do
            local neighbor_key = (cords[1] + dir[1]).."_"..(cords[2] + dir[2]).."_"..(cords[3] + z)
        
            -- Only include if we've discovered it (it's in self.nodes)
            if self.nodes[neighbor_key] then
                table.insert(neighbors, neighbor_key)
            end
        end
    end
    
    return neighbors
end

function MoveGoal:remove_from_open_set(open_set, key)
    for i, k in ipairs(open_set) do
        if k == key then
            table.remove(open_set, i)
            return
        end
    end
end

function MoveGoal:in_open_set(open_set, key)
    for _, k in ipairs(open_set) do
        if k == key then
            return true
        end
    end
    return false
end

function MoveGoal:compute_path(drone_id)
    local drone_cords = DroneFunctions.get_drone_cords(drone_id);
    
    local open_set = {}
    local closed_set = {}
    local came_from = {}

    -- Start at the drones current position and set already explored
    local start_key = drone_cords[1].."_"..drone_cords[2].."_"..drone_cords[3]
    closed_set[start_key] = true

    -- Add neighbors to open set
    local starting_nabors = self:get_neighbors_with_node_cords(drone_cords)
    for _, neighbor_key in ipairs(starting_nabors) do
        -- if has not been added to set before
        if not closed_set[neighbor_key] and not self:in_open_set(open_set, neighbor_key) then
            -- Add nabor to open set
            table.insert(open_set, neighbor_key)
            came_from[neighbor_key] = start_key
        end
    end

    -- A* loop
    local best_node_key = nil  -- Track closest node to goal
    local best_distance = math.huge

    while #open_set > 0 do
        -- Get best node from open set
        local current_key = self:get_lowest_h_value_node_key(open_set)
        if current_key == nil then
            print("ERROR: No valid node in open set")
            return nil
        end 

        local current_node = self.nodes[current_key]

        -- Check if we reached the goal
        if self:is_at_goal(current_key) then
            return self:reconstruct_path(came_from, current_key, start_key)
        end

        -- Track closest node to goal
        if current_node["distance_from_goal"] < best_distance then
            best_distance = current_node["distance_from_goal"]
            best_node_key = current_key
        end

        -- Move from open to closed
        self:remove_from_open_set(open_set, current_key)
        closed_set[current_key] = true

        -- Add neighbors to open set
        local neighbors = self:get_neighbors_with_key(current_key)
        for _, neighbor_key in ipairs(neighbors) do
            -- if has not been added to set before
            if not closed_set[neighbor_key] and not self:in_open_set(open_set, neighbor_key) then
                -- Add nabor to open set
                table.insert(open_set, neighbor_key)
                came_from[neighbor_key] = current_key
            end
        end
    end

    -- If we get here, we explored everything visible but didn't reach goal
    -- Return path to closest node
    if best_node_key then
        return self:reconstruct_path(came_from, best_node_key, start_key)
    end

    return nil
end

--[[
~~~~~~~~~~~~~~~~~~~~~~~~
~~ Main goal controls ~~
~~~~~~~~~~~~~~~~~~~~~~~~
--]]

function MoveGoal:is_at_goal(node_key)
    local node = self.nodes[node_key]
    local cords = node["Cords"]
    
    -- Check if within 1 block of goal (or exact match)
    return math.abs(cords[1] - self.goal_cords[1]) <= 1 
       and math.abs(cords[2] - self.goal_cords[2]) <= 1
end

function MoveGoal:is_at_goal_cords(cords)
    -- Check if within 1 block of goal (or exact match)
    return math.abs(cords[1] - self.goal_cords[1]) <= 1 
       and math.abs(cords[2] - self.goal_cords[2]) <= 1
end

function MoveGoal:is_complete()
    return self.complete
end

function MoveGoal:tik(drone_id)
    local drone_cords = DroneFunctions.get_drone_cords(drone_id)
    -- Check if goal cords have been reached
    if self:is_at_goal_cords(drone_cords) then
        self.complete = true
        return false;
    end

    -- Scan the nodes around the drone    
    self:discover_visible_nodes(drone_id)

    -- Execute current path 
    if self.path then
        -- No valid path
        if #self.path == 0 then
            self.complete = true
            return false;
        end

        local path_cords = self.nodes[self.path[1]]["Cords"]
        -- Calculate movment directions
        local x_direction = (path_cords[1] - drone_cords[1])
        local y_direction = (path_cords[2] - drone_cords[2])
        local z_direction = (path_cords[3] - drone_cords[3])

        -- Move drone
        DroneFunctions.move(drone_id, x_direction, y_direction, z_direction)

        -- Remove current movment from path
        table.remove(self.path, 1)
        -- Clear path if all movments have been executed
        if #self.path == 0 then
            self.path = nil
        end
        return true;
    else --Calculate the next path to move
        local path = self:compute_path(drone_id)
        -- Set the current path to the one calculated
        if path then
            self.path = path
        end
    end

end

return MoveGoal
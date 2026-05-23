
--- @class Quarry class for managing the quarry structure
--- @field start_cords table 2D cords of starting location
--- @field end_cords table 2D cords of ending location
--- @field scale integer How large it is
--- @field current_depth integer current depth of quarry
local Quarry = {}
Quarry.__index = Quarry

--- @param start_cords table 2D cords of starting position
--- @param scale integer how large the Quarry will break
--- @return Quarry the quarry
function Quarry.new(start_cords, scale)
    local self = setmetatable({}, Quarry)

    self.start_cords = start_cords
    self.end_cords = {start_cords[1] + scale, start_cords[2] + scale}
    self.scale = scale
    self.current_depth = 0

    return self
end

--- comment
--- @param block_cords table 3D intiger cords of block
--- @return boolean in range
function Quarry:block_in_quarry_area(block_cords)
    local in_x_range = block_cords[1] >= self.start_cords[1] and block_cords[1] <= self.end_cords[1]
    local in_y_range = block_cords[2] >= self.start_cords[2] and block_cords[2] <= self.end_cords[2]
    return in_x_range and in_y_range

end
--- comment
--- @param block_cords table 3D intiger cords of block
--- @return boolean in depth range
function Quarry:block_in_quarry_depth(block_cords)
    local in_depth_range = self.current_depth <= block_cords[3]
    return in_depth_range

end

return Quarry
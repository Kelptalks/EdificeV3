local Quarry = {}
Quarry.__index = Quarry

--- @param start_cords table 3D cords of starting position
--- @param scale integer how large the Quarry will break
--- @param depth integer how deep the Quarry will break
--- @return table the quarry
function Quarry.new(start_cords, scale, depth)
    local self = setmetatable({}, Quarry)
    
    local start_cords = {}
    local scale = scale
    local depth = depth

    return self
end

return Quarry
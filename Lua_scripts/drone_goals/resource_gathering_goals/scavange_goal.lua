local DroneFunctions = require("rust_wrapper_functions.drone_functions")
local BlockTypes = require("rust_wrapper_functions.block_types")

local ScavangeGoal = {}  -- Module table
ScavangeGoal.__index = ScavangeGoal  -- Add this!

function ScavangeGoal.new()
    local self = setmetatable({}, ScavangeGoal)
    self.complete = false
    return self
end

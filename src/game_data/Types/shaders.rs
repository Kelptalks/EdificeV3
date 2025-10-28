
pub static TOTAL_SHADERS: u32 = 14;

#[repr(u16)]
#[derive(Copy, Clone, PartialEq)]
pub enum ShaderTriangle
{
    TopLeft = 0,
    TopRight = 1,
    LeftTop = 2,
    LeftBot = 3,
    RightTop = 4,
    RightBot = 5,
    TopTopLeft = 6,
    TopBotLeft = 7,
    TopTopRight = 8,
    TopBotRight = 9,
    LeftCenterLeft = 10,
    LeftCenterTop = 11,
    LeftCenterRight = 12,
    LeftCenterBot = 13,
}
impl ShaderTriangle {
    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn id_as_usize(&self) -> usize {
        *self as usize
    }

    pub fn get_shader_triangle_id(&self) -> f32 {
        *self as u16 as f32
    }

    pub fn from_id(id: u16) -> ShaderTriangle {
        if id <= ShaderTriangle::LeftCenterBot as u16 {
            return unsafe { std::mem::transmute(id) }
        }
        else {
            return ShaderTriangle::TopLeft;
        }
    }
}

#[repr(u16)]
#[derive(Copy, Clone)]  // Add these
pub enum BlockShaderType {
    None = 0,
    Selector = 1,
    Grey = 2,
    Green = 3,
    Red = 4,
}

impl BlockShaderType {
    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn get_total_shaders() -> u32 {
        return TOTAL_SHADERS;
    }

    pub fn id_as_usize(&self) -> usize {
        *self as usize
    }

    pub fn from_id(id: u16) -> BlockShaderType {
        if id <= BlockShaderType::Red as u16 {
            return unsafe { std::mem::transmute(id) }
        }
        else {
            return BlockShaderType::None;
        }
    }
}

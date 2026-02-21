#[repr(u16)]
#[derive(Copy, Clone, PartialEq)]

/*
#####################
## Block Triangles ##
#####################
This file is responsable for creating managing block triangles which are the diffrent splices



*/

pub enum BlockTriangle
{
    TopLeft = 0,
    TopRight = 1,
    LeftTop = 2,
    LeftBot = 3,
    RightTop = 4,
    RightBot = 5,
}

static LEFT_FACING: [bool; 6] = [
    true,
    false,
    false,
    true,
    true,
    false,
];


impl BlockTriangle {
    pub const PIXLE_REZ : u32 = 32;
    
    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn id_as_usize(&self) -> usize {
        *self as usize
    }

    pub fn is_left_pointing(&self) -> bool {
        return LEFT_FACING[self.id_as_usize()];
    }

    pub fn from_id(id: u16) -> BlockTriangle {
        if id <= BlockTriangle::RightBot as u16 {
            return unsafe { std::mem::transmute(id) }
        }
        else {
            return BlockTriangle::TopLeft;
        }
    }
}

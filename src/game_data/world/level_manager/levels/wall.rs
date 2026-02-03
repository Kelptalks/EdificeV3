use crate::game_data::{level_manager::level::Level, screen::Button, types::{BlockType, UITextures}};

pub struct Wall {
    name: String,

}

impl Wall {
    pub fn new() -> Wall {
        Wall {
            name: "Wall".to_string(),
        }
    }
}

impl Level for Wall {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn gen_level(&self, world: &mut crate::game_data::World) {
        // Platform
        for x in -50..10 {
            for y in -10..10 {
                world.set_world_value(BlockType::Grass.id_as_u16(), [x, y, -1]);
                world.set_world_value(BlockType::Stone.id_as_u16(), [x, y, -2]);
                world.set_world_value(BlockType::Stone.id_as_u16(), [x, y, -3]);
                world.set_world_value(BlockType::Stone.id_as_u16(), [x, y, -4]);
            }
        }

        // Wall
        for y in -10..10 {
            for z in -1..10 {
                world.set_world_value(BlockType::CobbleStone.id_as_u16(), [-40, y, z]);
            }
        }

    }
    
    fn get_level_select_button(&self) -> Button {
        let mut button = Button::new_blank(UITextures::ButtonCircle);
        button.set_block(BlockType::CobbleStone);
        button.set_text(self.name.clone());


        return button;
    }
}
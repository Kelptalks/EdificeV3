use crate::game_data::{level_manager::level::Level, screen::Button, types::BlockTexture};

pub struct FlatField {
    name: String,

}

impl FlatField {
    pub fn new() -> FlatField {
        FlatField {
            name: "Flat Field".to_string(),
        }
    }
}

impl Level for FlatField {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn gen_level(&self, world: &mut crate::game_data::World) {
        for x in -100..100 {
            for y in -100..100 {
                world.set_world_value(BlockTexture::Grass.id_as_u16(), [x, y, -1]);
                world.set_world_value(BlockTexture::Stone.id_as_u16(), [x, y, -2]);
                world.set_world_value(BlockTexture::Stone.id_as_u16(), [x, y, -3]);
                world.set_world_value(BlockTexture::Stone.id_as_u16(), [x, y, -4]);
            }
        }

    }
    
    fn get_level_select_button(&self) -> Button {
        let mut button = Button::new_blank();
        button.set_block(BlockTexture::Grass);
        button.set_text(self.name.clone());


        return button;
    }
}
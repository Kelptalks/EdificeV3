use rand::random_range;

use crate::game_data::{level_manager::level::Level, screen::Button, types::{BlockTexture, UITextures}};

pub struct Monoliths {
    name: String,
}

impl Monoliths {

    pub fn new() -> Monoliths{
        Monoliths {
            name: "Monoliths".to_string()
        }
    }

    pub fn gen_monolith(world: &mut crate::game_data::World, cords: [i32; 3]) {
        let x_size = random_range(2..8);
        let y_size = random_range(2..8);
        
        for x in 0..x_size {
            for y in 0..y_size {
                for z in 0..2 {
                let block_cords = [
                    cords[0] + x,
                    cords[1] + y,
                    cords[2] + z,
                ];
                world.set_world_value(BlockTexture::Stone.id_as_u16(), block_cords);
                }
            }
        }
    }

}


impl Level for Monoliths {


    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn gen_level(&self, world: &mut crate::game_data::World) {
        for x in -200..200 {
            for y in -200..200 {
                world.set_world_value(BlockTexture::Grass.id_as_u16(), [x, y, -1]);
                world.set_world_value(BlockTexture::Stone.id_as_u16(), [x, y, -2]);
                world.set_world_value(BlockTexture::Stone.id_as_u16(), [x, y, -3]);
                world.set_world_value(BlockTexture::Stone.id_as_u16(), [x, y, -4]);
                
                // Gen moniliths if they are not the center
                if x > 20 || x < -20 || y > 20 || y < -20 {
                    let num = random_range(0..50);
                    if num == 1 {
                        Monoliths::gen_monolith(world, [x, y, 0]);
                    }
                }
            }
        }
    }

    fn get_level_select_button(&self) -> Button {
        let mut button = Button::new_blank();
        button.set_block(BlockTexture::Stone);
        button.set_text(self.name.clone());

        return  button;
    }
}
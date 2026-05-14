use crate::game_data::{player_data::game_entity::{components::entity_components::EntityComponent, dynamic_entity_manager::dynamic_entity_manager::DynamicEntity, game_entity_manager::GameEntity}, tools::id_gen::IdGen};


static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct DynamicEntityPuff {
    pub id: u64,
}

impl DynamicEntityPuff {
    
    pub fn wrap_into_game_entity(self) -> GameEntity {
        DynamicEntity::Puff(self).wrap_into_game_entity()
    }
    
    pub fn new(cords: [i32; 3]) -> DynamicEntityPuff {
        let id = ID_GEN.new_id();
        

        DynamicEntityPuff {
            id
        }
    }


    pub fn get_components(self) -> Vec<EntityComponent> {
        Vec::new()
    }
}
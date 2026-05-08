use crate::game_data::{TextureManager, game_event_manager::{event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, texture_manager::texture_cashe::texture_cashe::CashedTextureID};

#[derive(Clone, PartialEq)]
pub enum TextureManagerEvent {
    FreeCashedTexture(CashedTextureID)
}

impl TextureManagerEvent{
    pub fn wrap_into_event(self) -> Event {
        RenderEvent::TextureManagerEvent(self).wrap_into_event()
    }

     pub fn execute_event(
        &self, 
        texture_manager: &mut TextureManager,
    ) {
        match self {
            TextureManagerEvent::FreeCashedTexture(cashed_texture_id) => {
                texture_manager.free_cashed_texture(*cashed_texture_id)
            },
        }
    }
}
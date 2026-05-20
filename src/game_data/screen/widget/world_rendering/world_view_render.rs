#![allow(dead_code)]
use std::time::Instant;
use crate::game_data::prof_record;


use crate::game_data::{
    TextureManager, chunk_tile_map_manager::chunk_render_data::ChunkRenderData, player_data::{cursor::cursor::Cursor, player_data::PlayerData}, screen::{
        ScreenData, iso_cord_tool, widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::PanelColor, widget::{Widget, WidgetType}, widget_properties::WidgetProperties, world_rendering::{area_rendering_manager::{area_rendering_manager::AreaRenderingManager, block_lair_manager::lair_block::LairBlockMod, ray_caster::casted_triangle::CastedTriangle}, tile_map::TileMapId, view_mode::ViewMode, world_view_data::WorldViewData}}
    }, texture_manager::texture::Texture, types::BlockTexture, world::world::{WorldEvent, SpriteRenderRequest}
};

use crate::game_data::game_event_manager::prelude::*;

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum ViewDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl ViewDirection {
    pub fn rotation_matrix(&self) -> [[i32; 2]; 2] {
        match self {
            ViewDirection::North => [[ 1,  0], [ 0,  1]],
            ViewDirection::East  => [[ 0,  1], [-1,  0]],
            ViewDirection::South => [[-1,  0], [ 0, -1]],
            ViewDirection::West  => [[ 0, -1], [ 1,  0]],
        }
    }

    pub fn from_id(id: u8) -> Self {
        match id {
            0 => ViewDirection::North,
            1 => ViewDirection::South,
            2 => ViewDirection::East,
            3 => ViewDirection::West,
            _ => ViewDirection::North,
        }
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn to_string(&self) -> String {
        match self {
            ViewDirection::North => "North".to_string(),
            ViewDirection::South => "South".to_string(),
            ViewDirection::East => "East".to_string(),
            ViewDirection::West => "West".to_string(),
        }
    }
}

 */

pub struct PlayWorldViewRender {
    widget_properties: WidgetProperties,
    overlay_panel: Box<WidgetType>,

    center_ndc: [f32; 2],

    // Input | Handling
    events: Vec<Event>,

    area_rendering_manager: AreaRenderingManager,
    lair_block_mods: Vec<LairBlockMod>,
    


    // Camera Motion
    camera_ndc_offset: [f32; 2],

    // Cached scale values
    ndc_block_scale: f32,
    ndc_tile_scale: f32,
    ndc_tile_half_scale: f32,
    ndc_draw_centering_offset: [f32; 3],

    // debug
    entitys_drawn: u32,
}

impl PlayWorldViewRender {
    
    pub fn new(parent_props: &WidgetProperties) -> PlayWorldViewRender {

        let mut wp = WidgetProperties::new_with_parent_props(parent_props);
        wp.prefered_scale = [1.0; 2];
        wp.internal_buffers = [0.012; 4];


        

        PlayWorldViewRender {
            widget_properties: wp,
            overlay_panel: Box::new(Panel::new_blank().wrap_into_widget()),

            center_ndc: [0.0; 2],

            events: Vec::new(),

            area_rendering_manager: AreaRenderingManager::new(),
            lair_block_mods: Vec::new(),

            camera_ndc_offset: [0.0, 0.0],

            ndc_block_scale: 1.0,
            ndc_tile_scale: 1.0,
            ndc_tile_half_scale: 1.0,
            ndc_draw_centering_offset: [0.0; 3],

            // debug
            entitys_drawn: 0,
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::PlayWorldViewRender(self)
    }

    //=====================================
    // Controls
    //=====================================

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn add_events(&mut self, events: &mut Vec<Event>) {
        self.events.append(events);
    }

    fn handle_camera_zooming(
        &mut self, 
        screen_data: &ScreenData, 
        event_manager: &mut EventManager, 
        player_data: &PlayerData
    ) {
        let mut cursor_scheduler = player_data.get_cursor_event_scheduler();
        if screen_data.get_input_manager().get_mouse_input_data().scrolled_up() {
            cursor_scheduler.zoom_in()
        }
        else if screen_data.get_input_manager().get_mouse_input_data().scrolled_down() {
            cursor_scheduler.zoom_out()
        }
        cursor_scheduler.schedul_events(event_manager);
    }

    fn handle_camera_panning(
        &mut self, 
        screen_data: &ScreenData, 
        event_manager: &mut EventManager, 
        player_data: &PlayerData
    ) {
        


        let _cursor = player_data.get_cursor();

        let mut cursor_scheduler = player_data.get_cursor_event_scheduler();

        // Key panning
        // Z Axis || Up and down
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::Space) {
            cursor_scheduler.mod_cords([0, 0, 1]);
        }
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::LeftShift) {
            cursor_scheduler.mod_cords([0, 0, -1]);
        }

        // X Axis 
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::A) {
            cursor_scheduler.mod_cords([-1, 0, 0]);
        }
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::D) {
            cursor_scheduler.mod_cords([1, 0, 0]);
        }

        // Y Axis 
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::W) {
            cursor_scheduler.mod_cords([0, -1, 0]);
        }
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::S) {
            cursor_scheduler.mod_cords([0, 1, 0]);
        }


        // Middle mouse button panning
        if screen_data.is_middle_mouse_held() {
            let scrolling_offset = screen_data.get_change_in_mouse_ndc();
            self.camera_ndc_offset[0] += scrolling_offset[0];
            self.camera_ndc_offset[1] += scrolling_offset[1];
        }

        let cords_offset = [0; 3];

        let _iso_offset =
            iso_cord_tool::ndi_screen_cords_to_iso_cords(
                self.ndc_tile_scale, 
                self.camera_ndc_offset
            );


        cursor_scheduler.mod_cords(cords_offset);
        cursor_scheduler.schedul_events(event_manager);
    }

    fn get_mouse_triangle(
        &mut self,
        screen_data: &ScreenData,
        _event_manager: &mut EventManager,
        player_data: &PlayerData
    ) -> Option<CastedTriangle> {
        let mut mouse_ndc = screen_data.get_mouse_ndc();

        mouse_ndc[0] -= self.ndc_draw_centering_offset[0];
        mouse_ndc[1] -= self.ndc_draw_centering_offset[1];
        mouse_ndc[0] += self.camera_ndc_offset[0];
        mouse_ndc[1] += self.camera_ndc_offset[1];

        let iso = iso_cord_tool::ndi_screen_cords_to_iso_cords(self.ndc_block_scale, mouse_ndc);

        let tile_key = [iso[0].floor() as i32, iso[1].floor() as i32];
        let cz = player_data.get_cursor().get_cords()[2];
        let world_cords = [tile_key[0] + cz, tile_key[1] + cz, cz];

        let world_arc = player_data.get_world_ref();
        let world = world_arc.read().unwrap();
        let tiles = world.chunk_tile_set_manager.get_obscuring(world_cords, 3);

        let tile = tiles.into_iter().filter(|t| t.struck()).last()?;

        let frac_x = iso[0] - iso[0].floor();
        let frac_y = iso[1] - iso[1].floor();
        if frac_x > frac_y {
            Some(tile.get_right_triangle().clone())
        } else {
            Some(tile.get_left_triangle().clone())
        }
    }



    pub fn size_play_view(&mut self, cursor: &Cursor) {
        self.widget_properties.scale_based_off_parent();

        let scale = self.widget_properties.scale;

        
        let largest_side_of_location = 1;
        let block_diementions = largest_side_of_location + (cursor.get_zoom() as usize) * 2 + 1;

        self.ndc_block_scale = (scale[0] / block_diementions as f32) / 2.0;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;

        self.center_ndc = [0.0, 0.0];

        self.ndc_draw_centering_offset[0] = -self.ndc_block_scale;
        self.ndc_draw_centering_offset[1] = -self.ndc_block_scale / 2.0;
    }

    fn render_full_view(
        &mut self,
        texture_manager: &mut TextureManager,
        _screen_data: &ScreenData,
        event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {

        
        self.size_play_view(player_data.get_cursor());

        let world_arc = player_data.get_world_ref();
        let mut world = world_arc.write().unwrap();
    

        let cursor = player_data.get_cursor();

        self.camera_ndc_offset = iso_cord_tool::world_pos_to_ndc_cords(
            self.ndc_block_scale, 
            iso_cord_tool::world_cords_to_world_pos(cursor.get_cords())
        );
        


        let mut draw_cords = [0.0; 2];
        draw_cords[0] += self.ndc_draw_centering_offset[0];
        draw_cords[1] += self.ndc_draw_centering_offset[1];

        draw_cords[0] -= self.camera_ndc_offset[0];
        draw_cords[1] -= self.camera_ndc_offset[1];

        let chunk_render_data = ChunkRenderData::new(self.ndc_block_scale, draw_cords, cursor.get_cords(), cursor.get_zoom());


        texture_manager.update_expander_cache(self.ndc_block_scale);
        let t = Instant::now();
        world.render_world(
            player_data,
            texture_manager,
            &chunk_render_data,
        );
        
        prof_record("  world_render_world", t.elapsed());
        

        // Use personal tile map
        
        if ((cursor.get_zoom() as usize) * 2) < 32 {
            
        }
         
        

        // Debug Data
        let chunk_debug_data = world.get_chunk_debug_data(cursor.get_cords());
        let debug = event_manager.get_mut_debug_data();
        debug.clear("World");
        debug.record("World", "## CURSORS CHUNK ##".to_string());
        for data in chunk_debug_data {
            debug.record("World", data);
        }
        
        
        
    }

    //=====================================
    // Settings
    //=====================================

    pub fn set_prefered_size(&mut self, scale: f32) {
        self.widget_properties.prefered_scale = [scale; 2];
    }

}

impl Widget for PlayWorldViewRender {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.widget_properties.external_buffers = pos;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {
        let start = Instant::now();

        // Render View
        self.render_full_view(texture_manager, screen_data, event_manager, &player_data);
        self.lair_block_mods.clear();

        // Selector overlay — only when mouse is over a block that has a game entity
        if let Some(triangle) = self.get_mouse_triangle(screen_data, event_manager, player_data) {
            if triangle.has_struck_solid {
                let struck_cords = triangle.get_solid_block_struck_cords();
                let has_entity = player_data.get_world_ref()
                    .read().unwrap()
                    .get_block_entity(struck_cords)
                    .is_some();
                if has_entity {
                    event_manager.add_event(WorldEvent::RenderSprite(SpriteRenderRequest {
                        world_pos: [struck_cords[0] as f32, struck_cords[1] as f32, struck_cords[2] as f32],
                        texture: Texture::BlockTexture(BlockTexture::Selector),
                    }).wrap_into_event());
                }
            }
        }

        if let Some(view_mode) = player_data.get_view_mode() {
            match view_mode {
                ViewMode::God() => {
                    // Handle Visuals
                    let mut panel = Panel::new_blank();
                    panel.set_parent_pos(screen_data.get_viewport_uv());
                    panel.set_color(PanelColor::Clear);
                    panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);


                    // Handle Controls
                    self.handle_camera_panning(screen_data, event_manager, player_data);
                    self.handle_camera_zooming(screen_data, event_manager, player_data);
            
                    if let Some(mouse_triangle) = self.get_mouse_triangle(screen_data, event_manager, player_data) {
                        let object_cords = mouse_triangle.get_first_block_cords_struck();

                        let world_ref = player_data.current_world.try_read();
                        let object = match world_ref {
                            Ok(world) => world.get_block_entity(object_cords),
                            Err(_) => todo!(),
                        };

                        if let Some(object) = object {
                            if screen_data.was_left_released() {
                                player_data.game_entity_manager.open_entity_window(event_manager, object);
                                event_manager.add_event(
                                    PlayerDataEvent::SetViewMode(Some(ViewMode::GameObjectSpectate(object))).wrap_into_event()
                                );
                            }
                        }
                    }

                    let cursor = player_data.get_cursor();
                    
                    /* self.tile_map_manager.render_enitity_at_world_pos(
                        texture_manager, 
                        player_data, 
                        cursor.get_pos(),
                        Texture::BlockTexture(BlockTexture::Selector)
                    );
                    */

                    panel.size();

                    self.overlay_panel = Box::new(panel.wrap_into_widget());
                }
                ViewMode::GameObjectSpectate(id) => {
                    // Handle Controls
                    self.handle_camera_zooming(screen_data, event_manager, player_data);

                    // Escape exits spectate centrally, so a broken entity handler can't trap you here.
                    if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::Escape) {
                        event_manager.add_event(
                            PlayerDataEvent::SetViewMode(Some(ViewMode::God())).wrap_into_event()
                        );
                        self.overlay_panel = Box::new(Panel::new_blank().wrap_into_widget());
                    }
                    else if let Some(entity) = player_data.game_entity_manager.clone_game_entity(id) {
                        let mouse_triangle = self.get_mouse_triangle(screen_data, event_manager, player_data);
                        let world_view_data = WorldViewData::new(mouse_triangle);

                        let world_arc = player_data.get_world_ref();
                        let world = world_arc.read().unwrap();

                        let mut overlay = entity.play_view(&world_view_data, screen_data, &world, event_manager);
                        overlay.set_parent_pos(screen_data.get_viewport_uv());
                        overlay.size();
                        self.overlay_panel = Box::new(overlay);
                    }
                    else {
                        self.overlay_panel = Box::new(Panel::new_blank().wrap_into_widget());
                    }
                },
                ViewMode::Location(_location_id) => {

                },
            }
            self.overlay_panel.render(texture_manager, screen_data, event_manager, player_data);
        }
        else {
            self.handle_camera_panning(screen_data, event_manager, player_data);
            self.handle_camera_zooming(screen_data, event_manager, player_data);
            self.overlay_panel = Box::new(Panel::new_blank().wrap_into_widget());
        }
        


        // Debuging
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::O) {
            println!("clearing world");
            event_manager.add_event(WorldEvent::Clear.wrap_into_event());
        }


        let mut mouse_data = Vec::new();
        if let Some(triangle) = self.get_mouse_triangle(screen_data, event_manager, player_data) {
            let mouse_cords = triangle.get_solid_block_struck_cords();
            mouse_data.push(format!(
                "mouse_world_cords ({:?})", 
                mouse_cords,
            ));
            mouse_data.push("Triangle Textures".to_string());
            for texture in triangle.get_textures() {
                if let Texture::BlockTriangle(block, _triangle) = texture {
                    mouse_data.push(format!(" - {}", block.get_name()));
                }
            }
            mouse_data.push(format!("Block Depth({})", triangle.get_solid_block_depth()));
        }
        


        let debug = event_manager.get_mut_debug_data();
        debug.clear("Rendering");

        for data in mouse_data {
            debug.record("Rendering", data);
        }

        debug.record("Rendering", format!("Frame Time ({})ms", start.elapsed().as_secs_f32() * 1000.0));
        debug.record("Rendering", format!("Free Cashed Textures({})", texture_manager.get_mut_texture_cashe().total_free_textures()));
        debug.record("Rendering", format!("Entity's Drawn ({})", self.entitys_drawn));
        self.entitys_drawn = 0;        
        
    }


    
}
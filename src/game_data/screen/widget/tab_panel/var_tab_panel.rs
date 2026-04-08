use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::var::{self, game_vars::{dynamic_var::DynamicVar, game_var_type::GameVar, primitive_var::PrimitiveVar}, var_properties::{self, PropKey, PropValue, VarProperty}, var_type::{Var, VarTypeKind}}, screen::{widget::{self, drone_programming::vars::var_slot, panel::{panel::Panel, panel_texture_manager::PanelTextureManager}, prelude::{TabPanel, VarSlot}, scroll_panel::scroll_panel::ScrollPanel, tab_panel, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_calculations}, widget_properties::{self, WidgetProperties}}, types::drone_item::DroneItem};


pub struct VarTabPanel {
    widget_properties: WidgetProperties,
    
    var_ref: Rc<RefCell<Var>>,
    
    
    index_ref: Rc<RefCell<usize>>,

    var_slot: VarSlot,
    scroll_panel: ScrollPanel,
    panel_texture: PanelTextureManager,
}


impl VarTabPanel {

    

    pub fn new(var_ref: &Rc<RefCell<Var>>) -> VarTabPanel {
        
        let var_slot = VarSlot::new(var_ref);

        let index_ref = Rc::new(RefCell::new(0));

        let mut scroll_panel = ScrollPanel::new();

        let mut var_tab_panel = VarTabPanel {
            widget_properties: WidgetProperties::new_blank(),

            var_ref: var_ref.clone(),


            index_ref: index_ref.clone(),
            

            var_slot: var_slot,
            scroll_panel,
            panel_texture: PanelTextureManager::new(),
        };

        var_tab_panel.widget_properties.internal_buffers = [0.01; 4];
        var_tab_panel.set_prefered_scale([0.5; 2]);

        return var_tab_panel;
    }
    
    pub fn set_prefered_scale(&mut self, prefered_scale: [f32; 2]) {
        self.widget_properties.prefered_scale = prefered_scale;
    }



    fn get_widgets_for_var(&mut self) -> Vec<WidgetType>{
        let mut widgets = Vec::new();
        
        let var_properties = self.var_ref.borrow().get_properties();
        for prop in var_properties {
            widgets.push(prop.into_widget());
        }

        widgets
    }

    
}



impl Widget for VarTabPanel {
    fn get_pos(&self) -> [f32; 4] {
        self.widget_properties.pos
    }

    fn get_scale(&self) -> [f32; 2] {
        self.widget_properties.scale
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.widget_properties.prefered_scale
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.widget_properties.external_buffers = pos;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        
        self.widget_properties.scale_based_off_parent();

        self.var_slot.set_parent_pos(self.get_pos());
        self.scroll_panel.set_parent_pos(self.get_pos());

        
        // calculate buffers
        let var_prefered_size = self.var_slot.get_preffered_scale();
        let x_centering_buffer = (self.get_scale()[0] - var_prefered_size[0]) / 2.0;
        let buffer = [
            x_centering_buffer,
            0.0,
            x_centering_buffer,
            self.get_scale()[1] - var_prefered_size[1],
        ];
        self.var_slot.set_buffers(buffer);
        self.var_slot.size();

        // calculate buffers
        let buffer = [
            self.widget_properties.internal_buffers[0],
            self.widget_properties.internal_buffers[1] + var_prefered_size[1],
            self.widget_properties.internal_buffers[2],
            self.widget_properties.internal_buffers[3],
        ];
        self.scroll_panel.set_buffers(buffer);
        self.scroll_panel.size();

        self.panel_texture.size(self.get_pos(), self.get_scale());
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        
        // Set tab index based off var type
        self.scroll_panel.clear_widgets();
        
        let widgets = self.get_widgets_for_var();
        self.scroll_panel.add_widgets(widgets);
        

        texture_manager.render_ui_element_with_pos(crate::game_data::types::UITextures::VoidBackground, self.get_pos());

        self.panel_texture.render(texture_manager);
        self.var_slot.render(texture_manager, screen_data, game_event_manager);
        self.scroll_panel.render(texture_manager, screen_data, game_event_manager);


        
    }
}

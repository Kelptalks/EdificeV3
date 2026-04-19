use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::game_data::{game_event_manager::{self, event_manager, player_data_event_manager::var_event_manager::var_events::VarEvents, prelude::{EventManager, PlayerDataEvent}}, player_data::drone_script::var::{self, game_vars::{dynamic_var::DynamicVarType, game_var_type::GameVarType, primitive_var::PrimitiveGameVarType}, var::Var, var_properties::{self, PropKey, PropValue, VarProperty}, var_type::{self, VarKind, VarType}}, screen::widget::{self, drone_programming::var_slot::var_slot, panel::{panel::Panel, panel_texture_manager::PanelTextureManager}, prelude::{TabPanel, VarSlot}, scroll_panel::scroll_panel::ScrollPanel, tab_panel, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties}, types::drone_item::DroneItem};


pub struct PropWidgetPool {
    pub mutable: HashMap<PropKey, Rc<RefCell<WidgetType>>>,
    pub non_mutable: HashMap<PropKey, Rc<RefCell<WidgetType>>>,
}

impl PropWidgetPool {
    pub fn new() -> PropWidgetPool {
        PropWidgetPool {
            mutable: HashMap::new(),
            non_mutable: HashMap::new(),
        }
    }

    pub fn create_widgets_for_all_keys(&mut self) {
        let prop_keys = PropKey::get_all();
        for key in prop_keys {
            self.non_mutable.insert(key, Rc::new(RefCell::new(key.create_widget(false))));
            self.mutable.insert(key, Rc::new(RefCell::new(key.create_widget(true))));
        }
    }

}

pub struct VarTabPanel {
    widget_properties: WidgetProperties,

    var_ref: Rc<RefCell<VarType>>,

    var_slot: VarSlot,
    scroll_panel: ScrollPanel,
    panel_texture: PanelTextureManager,

    widget_pool: PropWidgetPool,
}


impl VarTabPanel {

    pub fn new() -> VarTabPanel {

        let var = Var::new_blank();
        let var_ref = var.get_var_type_ref();
        let var_slot = VarSlot::new_with_var(var);

        let scroll_panel = ScrollPanel::new();

        let mut var_tab_panel = VarTabPanel {
            widget_properties: WidgetProperties::new_blank(),
            var_ref: var_ref,

            var_slot: var_slot,
            scroll_panel,
            panel_texture: PanelTextureManager::new(),

            widget_pool: PropWidgetPool::new(),
        };

        var_tab_panel.widget_pool.create_widgets_for_all_keys();

        var_tab_panel.widget_properties.internal_buffers = [0.01; 4];
        var_tab_panel.set_prefered_scale([0.5; 2]);

        return var_tab_panel;
    }

    pub fn set_prefered_scale(&mut self, prefered_scale: [f32; 2]) {
        self.widget_properties.prefered_scale = prefered_scale;
    }

    fn get_widgets_for_var(&self, game_event_manager: &mut EventManager) -> Vec<Rc<RefCell<WidgetType>>> {
        let mut widgets = Vec::new();

        let var_properties = self.var_ref.borrow().get_properties();
        for prop in var_properties {

            let widget_option;
            if prop.mutible {
                widget_option = self.widget_pool.mutable.get(&prop.key);
            } else {
                widget_option = self.widget_pool.non_mutable.get(&prop.key);
            }

            if let Some(widget) = widget_option {
                let requests = prop.value.update_widget(&mut widget.borrow_mut());

                let mut events = Vec::new();

                for request in requests {
                    events.push(VarEvents::RequestEvent(self.var_ref.clone(), request).wrap_into_event());
                }

                game_event_manager.add_events(&events);

                widgets.push(widget.clone());
            }
        }

        widgets
    }


}



impl Widget for VarTabPanel {
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
        self.widget_properties.scale_based_off_parent();

        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;
        let internal_buffers = self.widget_properties.internal_buffers;

        self.var_slot.set_parent_pos(pos);
        self.scroll_panel.set_parent_pos(pos);

        let var_prefered_size = self.var_slot.get_preffered_scale();
        let x_centering_buffer = (scale[0] - var_prefered_size[0]) / 2.0;
        let buffer = [
            x_centering_buffer,
            0.0,
            x_centering_buffer,
            scale[1] - var_prefered_size[1],
        ];
        self.var_slot.set_buffers(buffer);
        self.var_slot.size();

        let buffer = [
            internal_buffers[0],
            internal_buffers[1] + var_prefered_size[1],
            internal_buffers[2],
            internal_buffers[3],
        ];
        self.scroll_panel.set_buffers(buffer);
        self.scroll_panel.size();

        self.panel_texture.size(pos, scale);
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
    ) {
        let bounds = self.widget_properties.bounds;

        self.scroll_panel.clear_widgets();

        texture_manager.render_ui_element_with_pos(crate::game_data::types::UITextures::VoidBackground, self.widget_properties.pos);

        self.panel_texture.render(texture_manager, bounds);

        self.var_slot.get_mut_widget_properties().bounds = bounds;
        self.var_slot.render(texture_manager, screen_data, game_event_manager);

        let widgets = self.get_widgets_for_var(game_event_manager);

        self.scroll_panel.get_mut_widget_properties().bounds = bounds;
        self.scroll_panel.render_shared_widgets(&widgets, texture_manager, screen_data, game_event_manager);
    }
}

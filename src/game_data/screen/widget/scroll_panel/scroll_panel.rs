use std::{cell::{RefCell}, rc::Rc};

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::GameEvent, prelude::EventManager, widget_event_manager::widget_event_manager::WidgetEvent}, player_data::player_data::PlayerData, screen::{ScreenData, widget::{bar_button::bar_button::BarButtonWidget, widget::{Widget, WidgetType}, widget_calculations, widget_properties::{WidgetId, WidgetProperties}}}};

pub struct ScrollPanel {
    widget_properties: WidgetProperties,

    // Widgets
    widgets: Vec<WidgetType>,

    // Scrolling
    scroll_value: Rc<RefCell<f32>>,
    max_scroll_value: f32,
    buttons: [BarButtonWidget; 2],
}

impl ScrollPanel {
    pub fn new() -> ScrollPanel {
        let scroll_ref = Rc::new(RefCell::new(0.0));
        let scroll_interval = 0.05;

        let mut scroll_up_button = BarButtonWidget::new("".to_string(), [0.0; 4]);
        scroll_up_button.add_event(GameEvent::WidgetEvent(WidgetEvent::Modf32Event(scroll_ref.clone(), -scroll_interval)));

        let mut scroll_down_button = BarButtonWidget::new("".to_string(), [0.0; 4]);
        scroll_down_button.add_event(GameEvent::WidgetEvent(WidgetEvent::Modf32Event(scroll_ref.clone(), scroll_interval)));

        let mut wp = WidgetProperties::new_blank();
        wp.prefered_scale = [0.2; 2];
        wp.internal_buffers = [0.001; 4];

        ScrollPanel {
            widget_properties: wp,

            widgets: Vec::new(),

            scroll_value: scroll_ref,
            max_scroll_value: 0.0,
            buttons: [scroll_up_button, scroll_down_button],
        }
    }

    pub fn add_widget(&mut self, widget: WidgetType) {
        self.widgets.push(widget);
    }

    pub fn add_widgets(&mut self, widgets: Vec<WidgetType>) {
        for widget in widgets {
            self.widgets.push(widget);
        }
    }

    pub fn get_mut_widgets(&mut self) -> &mut Vec<WidgetType> {
        &mut self.widgets
    }

    pub fn clear_widgets(&mut self) {
        self.widgets.clear();
    }

    pub fn find_widget_with_id(&mut self, id: WidgetId) -> Option<&mut WidgetType> {
        for widget in &mut self.widgets {
            if let Some(found) = widget.find_with_id(id) {
                return Some(found);
            }
        }
        None
    }

    pub fn set_prefered_scale(&mut self, scale: [f32; 2]) {
        self.widget_properties.prefered_scale = scale;

    }

}

impl Widget for ScrollPanel {
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
        let _pos_parent = self.widget_properties.parent_pos;
        let _ext_buffers = self.widget_properties.external_buffers;
        let internal_buffers = self.widget_properties.internal_buffers;

        self.widget_properties.scale_based_off_parent();
        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;

        let mut current_widget_buffer_offset = -*self.scroll_value.borrow();
        let mut total_content_height = 0.0;

        // Size Up Button
        let mut button_buffer = internal_buffers;
        button_buffer[3] += scale[1] - widget_calculations::get_button_scale() / 2.0;
        self.buttons[0].set_parent_pos(pos);
        self.buttons[0].set_buffers(button_buffer);
        self.buttons[0].size();
        let button_height = self.buttons[0].get_scale()[1];
        current_widget_buffer_offset += button_height;
        total_content_height += button_height;

        for widget in &mut self.widgets {
            let mut widget_buffer = internal_buffers;
            let widget_prefered_size = widget.get_preffered_scale();

            widget_buffer[2] += (scale[0] - widget_prefered_size[0]).max(0.0);
            widget_buffer[3] += scale[1] - widget_prefered_size[1];
            widget_buffer[1] += current_widget_buffer_offset;
            widget_buffer[3] -= current_widget_buffer_offset;

            widget.set_parent_pos(pos);
            widget.set_buffers(widget_buffer);
            widget.size();

            let widget_scale = widget.get_scale();
            let added = widget_scale[1] + internal_buffers[1] + internal_buffers[3];
            current_widget_buffer_offset += added;
            total_content_height += added;
        }

        // Size Down Button
        let mut button_buffer = internal_buffers;
        button_buffer[1] += scale[1] - widget_calculations::get_button_scale() / 2.0;
        self.buttons[1].set_parent_pos(pos);
        self.buttons[1].set_buffers(button_buffer);
        self.buttons[1].size();

        self.max_scroll_value = (total_content_height - scale[1]).max(0.0);
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {
        if *self.scroll_value.borrow() < 0.0 {
            *self.scroll_value.borrow_mut() = 0.0;
        } else if *self.scroll_value.borrow() > self.max_scroll_value {
            *self.scroll_value.borrow_mut() = self.max_scroll_value;
        }

        self.size();

        let pos = self.widget_properties.pos;
        let outer_bounds = self.widget_properties.bounds;
        let clip_bounds = Some(pos);

        for widget in &mut self.widgets {
            if widget_calculations::is_pos_overlapping_pos(pos, widget.get_pos()) {
                widget.get_mut_widget_properties().bounds = clip_bounds;
                widget.render(texture_manager, screen_data, game_event_manager, player_data);
            }
        }

        for button in &mut self.buttons {
            button.get_mut_widget_properties().bounds = outer_bounds;
            button.render(texture_manager, screen_data, game_event_manager, player_data);
        }

        if screen_data.mouse_on_ndc_pos(pos) {
            let inputs = screen_data.get_inputs();
            for input in inputs {
                match input {
                    crate::game_data::screen::input_data::Input::MouseWheel(_x, y) => {
                        if *y > 0.0 {
                            game_event_manager.add_widget_event(WidgetEvent::Modf32Event(self.scroll_value.clone(), -0.03));
                        } else if *y < 0.0 {
                            game_event_manager.add_widget_event(WidgetEvent::Modf32Event(self.scroll_value.clone(), 0.03));
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

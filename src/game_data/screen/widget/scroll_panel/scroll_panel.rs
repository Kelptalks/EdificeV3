use std::{cell::{Ref, RefCell}, rc::Rc};

use image::buffer;

use crate::game_data::{game_event_manager::{game_event_manager::Event, widget_event_manager::widget_event_manager::WidgetEvent}, screen::widget::{self, bar_button::bar_button::BarButtonWidget, button::button::Button, widget::{Widget, WidgetType}, widget_calculations}};

pub struct ScrollPanel {
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Self Rendering
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],

    // Widgets
    panels: Vec<WidgetType>,

    // Scrolling
    current_scroll: Rc<RefCell<f32>>,
    buttons: [BarButtonWidget; 2],

}

impl ScrollPanel {
    pub fn new() -> ScrollPanel {
        let scroll_ref = Rc::new(RefCell::new(0.0));
        let scroll_interval = 0.05;

        let mut scroll_up_button = BarButtonWidget::new("".to_string(), [0.0; 4]);
        scroll_up_button.add_event(Event::WidgetEvent(WidgetEvent::Modf32Event(scroll_ref.clone(), scroll_interval)));
        
        let mut scroll_down_button = BarButtonWidget::new("".to_string(), [0.0; 4]);
        scroll_down_button.add_event(Event::WidgetEvent(WidgetEvent::Modf32Event(scroll_ref.clone(), -scroll_interval)));

        ScrollPanel {
            // Parent Rendering
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [0.2; 2],

            // Self Rendering
            external_buffers: [0.0; 4], 
            internal_buffers: [0.0; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],

            // Widgets
            panels: Vec::new(),

            // Scrolling
            current_scroll: scroll_ref,

            buttons: [scroll_up_button, scroll_down_button],
        }
    }


    pub fn add_panel(&mut self, widget: WidgetType) {
        self.panels.push(widget);
    }

    pub fn set_prefered_scale(&mut self, scale: [f32; 2]) {
        self.prefered_scale = scale;
    }
}

impl Widget for ScrollPanel {
    fn get_pos(&self) -> [f32; 4] {
        self.pos
    }

    fn get_scale(&self) -> [f32; 2] {
        self.scale
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.prefered_scale
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.external_buffers = pos;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
    }

    fn size(&mut self) {
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);


        let mut current_widget_buffer_offset = *self.current_scroll.borrow();
        self.buttons[0].set_parent_pos(self.pos);


        // Size Up Button
        let mut button_buffer = self.internal_buffers;
        button_buffer[3] += self.scale[1] - widget_calculations::get_button_scale() / 2.0;
        self.buttons[0].set_parent_pos(self.pos);
        self.buttons[0].set_buffers(button_buffer);
        self.buttons[0].size();
        current_widget_buffer_offset += self.buttons[0].get_scale()[1];


        for widget in &mut self.panels {
            let mut widget_buffer = self.internal_buffers;
            
            let widget_prefered_size = widget.get_preffered_scale();

            // Calculate buffers based off widget size
            widget_buffer[2] += self.scale[0] - widget_prefered_size[0];
            widget_buffer[3] += self.scale[1] - widget_prefered_size[1];

            // Offset widget based off pos in window
            widget_buffer[1] += current_widget_buffer_offset; 
            widget_buffer[3] -= current_widget_buffer_offset; 
            
            widget.set_parent_pos(self.pos);
            widget.set_buffers(widget_buffer);
            widget.size();


            let widget_scale = widget.get_scale();
            current_widget_buffer_offset += widget_scale[1] + self.internal_buffers[1] + self.internal_buffers[3];
        }

        // Size Down Button
        let mut button_buffer = self.internal_buffers;
        button_buffer[1] += self.scale[1] - widget_calculations::get_button_scale() / 2.0;
        self.buttons[1].set_parent_pos(self.pos);
        self.buttons[1].set_buffers(button_buffer);
        self.buttons[1].size();
        
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::game_event_manager::GameEventManager
    ) {
        
        texture_manager.render_ui_element_with_pos(crate::game_data::types::UITextures::MirrorBackground, self.pos);

        // Need to size every frame due to scrolling
        self.size();

        for widget in &mut self.panels {
            if let WidgetType::Panel(panel) = widget {
                // If panel is fully contained within the widgets area
                if widget_calculations::is_pos_contained_within_pos(self.pos, panel.get_pos()) {
                    panel.render(texture_manager, screen_data, game_event_manager);
                }
            }
        }

        for button in &mut self.buttons {
            button.render(texture_manager, screen_data, game_event_manager);
        }

        if screen_data.mouse_on_ndc_pos(self.pos) {
            let inputs = screen_data.get_inputs();
            for input in inputs {
                match input {
                    crate::game_data::screen::input_data::Input::MouseWheel(x, y) => {
                        if *y > 0.0 {
                            game_event_manager.add_widget_event(WidgetEvent::Modf32Event(self.current_scroll.clone(), 0.01));
                        }
                        else if *y < 0.0 {
                            game_event_manager.add_widget_event(WidgetEvent::Modf32Event(self.current_scroll.clone(), -0.01));
                        }
                    
                    },
                    _ => {

                    }
                }
            }
        }

    }
}
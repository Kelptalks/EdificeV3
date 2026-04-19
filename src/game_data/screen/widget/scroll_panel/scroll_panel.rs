use std::{cell::{RefCell}, rc::Rc};

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::GameEvent, prelude::EventManager, widget_event_manager::widget_event_manager::WidgetEvent}, screen::{ScreenData, widget::{self, bar_button::bar_button::BarButtonWidget, widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties}}};

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

    pub fn render_shared_widgets(
        &mut self,
        widgets: &Vec<Rc<RefCell<WidgetType>>>,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
    ) {
        // Clamp scroll value
        if *self.scroll_value.borrow() < 0.0 {
            *self.scroll_value.borrow_mut() = 0.0;
        } else if *self.scroll_value.borrow() > self.max_scroll_value {
            *self.scroll_value.borrow_mut() = self.max_scroll_value;
        }

        self.size();

        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;
        let internal_buffers = self.widget_properties.internal_buffers;
        let outer_bounds = self.widget_properties.bounds;

        let mut offset = -*self.scroll_value.borrow() + self.buttons[0].get_scale()[1];

        for widget in widgets {
            let mut w = widget.borrow_mut();
            let mut widget_buffer = internal_buffers;
            let preferred = w.get_preffered_scale();

            widget_buffer[2] += (scale[0] - preferred[0]).max(0.0);
            widget_buffer[3] += scale[1] - preferred[1];
            widget_buffer[1] += offset;
            widget_buffer[3] -= offset;

            w.set_parent_pos(pos);
            w.set_buffers(widget_buffer);
            w.size();

            let widget_scale = w.get_scale();
            w.get_mut_widget_properties().bounds = Some(pos);
            w.render(texture_manager, screen_data, game_event_manager);

            offset += widget_scale[1] + internal_buffers[1] + internal_buffers[3];
        }

        self.max_scroll_value = offset;

        for button in &mut self.buttons {
            button.get_mut_widget_properties().bounds = outer_bounds;
            button.render(texture_manager, screen_data, game_event_manager);
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

    pub fn set_prefered_scale(&mut self, scale: f32) {
        self.widget_properties.prefered_scale[1] = scale;

        let mut largest_widget_prefered_x_scale = 0.0;
        for widget in &mut self.widgets {
            widget.size();
            let widget_prefered_size = widget.get_preffered_scale();
            if largest_widget_prefered_x_scale < widget_prefered_size[0] {
                largest_widget_prefered_x_scale = widget_prefered_size[0];
            }
        }
        self.widget_properties.prefered_scale[0] = largest_widget_prefered_x_scale + 0.001;
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
        let pos_parent = self.widget_properties.parent_pos;
        let ext_buffers = self.widget_properties.external_buffers;
        let internal_buffers = self.widget_properties.internal_buffers;

        self.widget_properties.scale_based_off_parent();
        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;

        let mut current_widget_buffer_offset = -*self.scroll_value.borrow();

        // Size Up Button
        let mut button_buffer = internal_buffers;
        button_buffer[3] += scale[1] - widget_calculations::get_button_scale() / 2.0;
        self.buttons[0].set_parent_pos(pos);
        self.buttons[0].set_buffers(button_buffer);
        self.buttons[0].size();
        current_widget_buffer_offset += self.buttons[0].get_scale()[1];

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
            current_widget_buffer_offset += widget_scale[1] + internal_buffers[1] + internal_buffers[3];
        }

        // Size Down Button
        let mut button_buffer = internal_buffers;
        button_buffer[1] += scale[1] - widget_calculations::get_button_scale() / 2.0;
        self.buttons[1].set_parent_pos(pos);
        self.buttons[1].set_buffers(button_buffer);
        self.buttons[1].size();

        self.max_scroll_value = current_widget_buffer_offset;
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
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
                widget.render(texture_manager, screen_data, game_event_manager);
            }
        }

        for button in &mut self.buttons {
            button.get_mut_widget_properties().bounds = outer_bounds;
            button.render(texture_manager, screen_data, game_event_manager);
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

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_programming::{compiled_script_element, function::{self, function::Function}, script_element::{self, ScriptElement}, var::{programming_vars::programming_var::{self, ProgrammingVar}, var_type::Var}}, drones::drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, getter_actions::getter_actions::DroneGetterAction, prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}}}, screen::{ScreenData, screen_data, ui_elements::panel, widget::{self, drone_programming::function_slot::FunctionSlot, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::VarSlot, scroll_panel::{self, scroll_panel::ScrollPanel}, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_calculations::{self, buffer_pos}}}};

pub struct ScriptingPanel {
    panel : Panel,
    scroll_panel: ScrollPanel,

    root_function: Rc<RefCell<Function>>
}

impl ScriptingPanel {
    pub fn new() -> ScriptingPanel {
        let mut panel = Panel::new_blank();

        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display("Scripting Panel".to_string());
        

        // TESTING 
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        let mut function = Function::new_blank();
        let mut sub_function = Function::new_from_drone_action(DroneAction::GetterAction(DroneGetterAction::IsBusy));
        let return_values = sub_function.get_return();
        for (return_value, script_element) in return_values {
            if let ScriptElement::Function(script_element) = script_element{
                let test_function = Function::new_from_drone_action(DroneAction::PrimAction(DronePrimAction::DroneWorldAction(DroneWorldAction::MineBlock([0, 0, 1]))));
                script_element.borrow_mut().add_script_element(0, test_function.clone().to_script_element());
                script_element.borrow_mut().add_script_element(0, test_function.clone().to_script_element());
                script_element.borrow_mut().add_script_element(0, test_function.clone().to_script_element());

                // sub function
                let mut sub_sub_function = Function::new_from_drone_action(DroneAction::GetterAction(DroneGetterAction::IsBusy));
                

                let sub_return_values = sub_sub_function.get_return();

                for (sub_return_values, sub_script_element) in sub_return_values {
                    if let ScriptElement::Function(sub_script_element) = sub_script_element {
                        sub_script_element.borrow_mut().add_script_element(0, test_function.clone().to_script_element());
                    }
                }

                script_element.borrow_mut().add_script_element(0, sub_sub_function.to_script_element());

            }
            
        }
        function.add_script_element(0, sub_function.to_script_element());
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        


        ScriptingPanel {
            
            panel: panel,

            scroll_panel: ScrollPanel::new(),


            root_function: Rc::new(RefCell::new(function))
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        return WidgetType::ScriptingPanel(self)
    }


    fn handle_released_var(&mut self, var_held_by_mouse: &Rc<RefCell<Var>>) {
        let borrow = var_held_by_mouse.borrow_mut();
        if let Var::ProgrammingVar(programming_var) = &*borrow {
            if let ProgrammingVar::Function(function) = programming_var{

            }
        }
    }


    //=====================================
    // Script Rendering
    //=====================================

    fn get_refreshed_scripting_widgets(&mut self) -> Vec<WidgetType> {
        let flattened_script = self.root_function.borrow_mut().flatten_functions(0);
        
        // Init widgets
        let mut widgets = Vec::new();


        for (indent, flattened_element) in flattened_script {

            if let ScriptElement::Action(_) = flattened_element {
                
            }
            else {
                let mut indented_panel = Panel::new_blank();
                indented_panel.add_text_display("-".repeat(indent));
                indented_panel.add_widget(flattened_element.construct_widget());
                widgets.push(indented_panel.wrap_into_widget());
            }

        }

        widgets
    }

    //=====================================
    // Script Editing
    //=====================================

    fn get_mouse_script_index(&mut self, screen_data: &ScreenData) -> Option<usize> {
        let widgets = self.scroll_panel.get_mut_widgets();
        let mut widget_current_offset = 0.0; 
        for (i, widget) in widgets.iter_mut().enumerate() {
            if let WidgetType::Panel(panel) = widget {
                if panel.is_mouse_on() {
                    return Some(i);
                }
            }
        }
        return None;
    }


}

impl Widget for ScriptingPanel {
    fn get_pos(&self) -> [f32; 4] {
        self.panel.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.panel.get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        let scroll_scale = self.scroll_panel.get_preffered_scale();
        let panel_scale = self.panel.get_preffered_scale();

        let x_scale = 0.5; // scroll_scale[0].max(panel_scale[0]); // Largest X
        let y_scale = scroll_scale[1] + panel_scale[1]; // Add Y 
    
        [x_scale, y_scale]
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        let panel_scale = self.panel.get_preffered_scale();

        let mut scroll_buffer: [f32; 4] = pos;
        scroll_buffer[1] += panel_scale[1];



        self.panel.set_buffers(pos);
        self.scroll_panel.set_buffers(scroll_buffer);        
    
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos);
        self.scroll_panel.set_parent_pos(pos);
    }

    fn size(&mut self) {
        self.panel.size();
        self.scroll_panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {

        

        if screen_data.mouse_on_ndc_pos(self.get_pos()) {
            // Add var
            if screen_data.was_left_released() {
                let var_held_by_mouse = game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().get_var_held();
                if let Some(var) = var_held_by_mouse {
                    self.handle_released_var(var);
                }
            }
        }

        let mouse_script_index = self.get_mouse_script_index(screen_data);


        self.scroll_panel.clear_widgets();
        
        
        // Add the widgets to scroll panel
        let widgets = self.get_refreshed_scripting_widgets();
        

        if let Some(mouse_script_index) = mouse_script_index {
            for (i, mut widget) in widgets.into_iter().enumerate() {
                
                if i == mouse_script_index {
                    if let WidgetType::Panel(panel) = &mut widget {
                        panel.set_color(widget::prelude::PanelColor::Dark);
                    }
                }

                self.scroll_panel.add_widget(widget);
            
            }
        }



        
        self.size();


        self.panel.render(texture_manager, screen_data, game_event_manager);
        self.scroll_panel.render(texture_manager, screen_data, game_event_manager);
        
    }
}
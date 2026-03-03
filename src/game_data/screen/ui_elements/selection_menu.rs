use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{Button, ScreenData, input_data::Input, ui_elements::panel::Panel}, types::UITextures};

pub struct SelectionMenu {
    ndc: [f32; 2],
    scale: [f32; 2],
    ndc_pos: [f32; 4],

    visibile: bool,

    // Page data
    button_selected_index: Option<usize>,
    buttons_per_page: i32,
    buttons_per_row: i32,
    buttons_per_col: i32,
    current_page: i32,
    total_pages: i32,

    panel: Panel,
    selection_buttons: Vec<Button>,

    button_close: Button,
    button_back: Button,
    button_foward: Button,
}

impl SelectionMenu {
    pub fn new() -> SelectionMenu {
        let mut panel = Panel::new_blank();
        panel.set_tile_ndc_scale(0.025);
        panel.set_title("title".to_string());
        SelectionMenu {
            ndc: [0.0; 2],
            scale: [0.0; 2],
            ndc_pos: [0.0; 4],

            visibile: true,

            // Page data
            button_selected_index: None,
            buttons_per_page: 20,
            buttons_per_row: 1,
            buttons_per_col: 1,
            current_page: 0,
            total_pages: 0,

            panel: panel,
            selection_buttons: Vec::new(),

            button_close: Button::new_blank_with_texture(UITextures::ButtonX),
            button_back: Button::new_blank_with_texture(UITextures::ButtonLeftArrow),
            button_foward: Button::new_blank_with_texture(UITextures::ButtonRightArrow),
        }
    }

    //=====================================
    // Rendering Getters / Setters
    //=====================================

    fn resizebuttons(&mut self) {
        let panel_start = self.panel.get_ndc();
        let panel_end = self.panel.get_ending_ndc();
        let text_end = self.panel.get_text_ending_ndc();
        let page_nav_scale = (text_end[1] - panel_start[1]) / 2.0;

        // Nav / close buttons
        self.button_back.set_scale(page_nav_scale);
        self.button_back.set_ndc([panel_start[0], panel_end[1] - page_nav_scale]);

        self.button_foward.set_scale(page_nav_scale);
        self.button_foward.set_ndc([panel_end[0] - page_nav_scale, panel_end[1] - page_nav_scale]);

        self.button_close.set_scale(page_nav_scale);
        self.button_close.set_ndc([panel_end[0] - page_nav_scale, panel_start[1]]);

        // Usable grid area (below title, above nav buttons)
        let grid_x = panel_start[0];
        let grid_y = text_end[1];
        let grid_w = panel_end[0] - panel_start[0];
        let grid_h = (panel_end[1] - page_nav_scale) - text_end[1];

        if grid_w <= 0.0 || grid_h <= 0.0 || self.buttons_per_page <= 0 {
            return;
        }

        // Derive rows/cols to match the grid aspect ratio
        let aspect = grid_w / grid_h;
        let col_f = ((self.buttons_per_page as f32) * aspect).sqrt();
        self.buttons_per_row = (col_f.round() as i32).max(1);
        self.buttons_per_col = ((self.buttons_per_page + self.buttons_per_row - 1) / self.buttons_per_row).max(1);

        // Largest square button that tiles evenly across the grid
        let btn_step = (grid_w / self.buttons_per_row as f32)
            .min(grid_h / self.buttons_per_col as f32);

        let button_padding = 0.1 * btn_step;
        let btn_scale = btn_step - button_padding;


        // Position every button by its index within its page
        for (i, button) in self.selection_buttons.iter_mut().enumerate() {
            let page_local = (i as i32) % self.buttons_per_page;
            let row = page_local / self.buttons_per_row;
            let col = page_local % self.buttons_per_row;
            button.set_scale(btn_scale);
            button.set_ndc([
                grid_x + col as f32 * btn_step + button_padding,
                grid_y + row as f32 * btn_step,
            ]);
        }
    }

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.panel.set_ndc(ndc);

        self.resizebuttons();
    }
    pub fn get_ndc(&self) -> [f32; 2]{
        return self.ndc;

        self.resizebuttons();
    }
    pub fn get_ending_ndc(&self) -> [f32; 2] {
        return [
            self.ndc[0] + self.scale[0],
            self.ndc[1] + self.scale[1],
        ];
    }

    pub fn set_scale(&mut self, ndc_scale: [f32; 2]) {
        self.scale = ndc_scale;
        self.panel.set_ndc_scale(ndc_scale);

        self.resizebuttons();
    }
    pub fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    pub fn is_visible(&self) -> bool {
        return self.visibile;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visibile = visible;
    } 

    //=====================================
    // Button Getters / Setters
    //=====================================

    pub fn get_mut_buttons(&mut self) -> &mut Vec<Button> {
        return &mut self.selection_buttons
    }

    pub fn add_button(&mut self, button: Button) {
        self.selection_buttons.push(button);
        self.total_pages = ((self.selection_buttons.len() as i32 - 1) / self.buttons_per_page).max(0);
        self.resizebuttons();
    }

    pub fn set_selected_button(&mut self, index: usize) {
        self.button_selected_index = Some(index);
    }

    pub fn clear_selected_button(&mut self) {
        self.button_selected_index = None;
    }

    pub fn get_selected_button_clone(&self) -> Option<Button> {
        self.button_selected_index
            .and_then(|i| self.selection_buttons.get(i))
            .cloned()
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        if !self.is_visible() {
            return;
        }
        
        
        self.panel.render(texture_manager);

        let page_num_string = format!("Page: {}", self.current_page);
        self.panel.set_title(page_num_string);

        // Render Back and foward buttons
        self.button_close.render_button(texture_manager, screen_data);
        if self.current_page > 0 {
            self.button_back.render_button(texture_manager, screen_data);
        }
        if self.current_page < self.total_pages {
            self.button_foward.render_button(texture_manager, screen_data);
        }


        // Render only buttons on the current page
        let page_start = (self.current_page * self.buttons_per_page) as usize;
        let page_end = ((self.current_page + 1) * self.buttons_per_page) as usize;
        let len = self.selection_buttons.len();
        for button in self.selection_buttons[page_start..page_end.min(len)].iter_mut() {
            button.render_button(texture_manager, screen_data);
        }

        // Input
        for input in screen_data.get_inputs() {
            match input {
                Input::MouseButtonDown(mouse_button) => {
                    if *mouse_button == MouseButton::Left {
                        if self.button_close.is_mouse_on_button() {
                            self.visibile = false;
                        }
                        else if self.button_back.is_mouse_on_button() && self.current_page > 0 {
                            self.current_page = (self.current_page - 1).max(0);
                        }
                        else if self.button_foward.is_mouse_on_button() && self.current_page < self.total_pages {
                            self.current_page = (self.current_page + 1).min(self.total_pages);
                        }

                        // Only check buttons on the current page
                        let clicked = self.selection_buttons[page_start..page_end.min(len)]
                            .iter()
                            .enumerate()
                            .find(|(_, b)| b.is_mouse_on_button())
                            .map(|(i, _)| page_start + i);
                        if let Some(idx) = clicked {
                            self.button_selected_index = Some(idx);
                        }
                    }
                }
                Input::MouseButtonUp(mouse_button) => {
                    if *mouse_button == MouseButton::Left {
                        // Only check buttons on the current page
                        self.button_selected_index = None;
                    }
                }
                _ => {}
            }
        }
    }
    
}
pub enum CursorMode {
    Free(), // Do not restric movment of cursor

    Locked(Rc<Ref<[i32; 3]>>), // Lock the cursor to world cords

    Expand(Rc<RefCell<WorldLocation>>), // Expand a location when it leaves it's bounds to fit the cursor
    LocationLocked(Rc<RefCell<WorldLocation>>), // Prevent the cursor from leaveing the locations bounds

}

impl CursorMode {
    pub fn get_id() -> usize {

    }

    pub fn from_id() -> usize {

    }
}


pub struct CursorConfig {
    location: Rc<RefCell<WorldLocation>>,
    cursor_mode: Rc<RefCell<usize>>,
}

impl CursorConfig {


    fn get_location_shift_events(&self) -> Vec<Event> {
        // if event type lock don't create events for shifting out of bounds

        // enum for location sides | get_sides_of_cord_in_location -> Vec<ZPlus, ZMinus>
        // Expand Side Event(ZPlus)
        // 

    }

    pub fn get_input_events(&self) -> Vec<Event> {
        match self.cursor_mode {
            // If free allow all movements
            
            // If locked set cords of locked possition

            // If Expand modify the expanded location based off cursor position

            // If Locked prevent the cursor from leaving the bounds of the position

        }
    }

    pub fn get_cursor_mode_ref(&self) -> Rc<RefCell<usize>> {
        
    }
    
}



use std::{cell::RefCell, rc::Rc};

use miniquad::KeyCode;

use crate::game_data::game_event_manager::prelude::{Event, InputEvent, StringEvent, UsizeEvent};

struct TypingInput {
    char: char,
    key_codes: Vec<KeyCode>,
}

impl TypingInput {
    fn new(char: char, key_codes: Vec<KeyCode>) -> TypingInput {
        TypingInput {char, key_codes}
    }

    fn get_event(self, string_ref: &Rc<RefCell<String>>, cursor_index_ref: &Rc<RefCell<usize>>) -> Event {
        let mut events = 
            StringEvent::InsertCharWithRefIndex(string_ref.clone(), cursor_index_ref.clone(), self.char).wrap_into_event_vec();
        
        events.insert(0, UsizeEvent::ModUsize(cursor_index_ref.clone(), 1).wrap_into_event());
        

        // Need to determin if a keyinput or keyinput should be used
        // Why: This is done to save on perfomance rather then using keyinputs for every event
        if self.key_codes.len() == 1 {
            return InputEvent::KeyDown(self.key_codes[0], events).wrap_into_event();
        }
        else {
            return InputEvent::KeysDown(self.key_codes, events).wrap_into_event();
        }
    }
}


fn get_typing_events(string_ref: &Rc<RefCell<String>>, cursor_index_ref: &Rc<RefCell<usize>>) -> Vec<Event> {
    let mut events = Vec::new();

    let typing_inputs = [
        // Letters (lowercase - no modifier)
        TypingInput::new('a', vec![KeyCode::A]),
        TypingInput::new('b', vec![KeyCode::B]),
        TypingInput::new('c', vec![KeyCode::C]),
        TypingInput::new('d', vec![KeyCode::D]),
        TypingInput::new('e', vec![KeyCode::E]),
        TypingInput::new('f', vec![KeyCode::F]),
        TypingInput::new('g', vec![KeyCode::G]),
        TypingInput::new('h', vec![KeyCode::H]),
        TypingInput::new('i', vec![KeyCode::I]),
        TypingInput::new('j', vec![KeyCode::J]),
        TypingInput::new('k', vec![KeyCode::K]),
        TypingInput::new('l', vec![KeyCode::L]),
        TypingInput::new('m', vec![KeyCode::M]),
        TypingInput::new('n', vec![KeyCode::N]),
        TypingInput::new('o', vec![KeyCode::O]),
        TypingInput::new('p', vec![KeyCode::P]),
        TypingInput::new('q', vec![KeyCode::Q]),
        TypingInput::new('r', vec![KeyCode::R]),
        TypingInput::new('s', vec![KeyCode::S]),
        TypingInput::new('t', vec![KeyCode::T]),
        TypingInput::new('u', vec![KeyCode::U]),
        TypingInput::new('v', vec![KeyCode::V]),
        TypingInput::new('w', vec![KeyCode::W]),
        TypingInput::new('x', vec![KeyCode::X]),
        TypingInput::new('y', vec![KeyCode::Y]),
        TypingInput::new('z', vec![KeyCode::Z]),

        // Letters (uppercase - shift modifier)
        TypingInput::new('A', vec![KeyCode::LeftShift, KeyCode::A]),
        TypingInput::new('B', vec![KeyCode::LeftShift, KeyCode::B]),
        TypingInput::new('C', vec![KeyCode::LeftShift, KeyCode::C]),
        TypingInput::new('D', vec![KeyCode::LeftShift, KeyCode::D]),
        TypingInput::new('E', vec![KeyCode::LeftShift, KeyCode::E]),
        TypingInput::new('F', vec![KeyCode::LeftShift, KeyCode::F]),
        TypingInput::new('G', vec![KeyCode::LeftShift, KeyCode::G]),
        TypingInput::new('H', vec![KeyCode::LeftShift, KeyCode::H]),
        TypingInput::new('I', vec![KeyCode::LeftShift, KeyCode::I]),
        TypingInput::new('J', vec![KeyCode::LeftShift, KeyCode::J]),
        TypingInput::new('K', vec![KeyCode::LeftShift, KeyCode::K]),
        TypingInput::new('L', vec![KeyCode::LeftShift, KeyCode::L]),
        TypingInput::new('M', vec![KeyCode::LeftShift, KeyCode::M]),
        TypingInput::new('N', vec![KeyCode::LeftShift, KeyCode::N]),
        TypingInput::new('O', vec![KeyCode::LeftShift, KeyCode::O]),
        TypingInput::new('P', vec![KeyCode::LeftShift, KeyCode::P]),
        TypingInput::new('Q', vec![KeyCode::LeftShift, KeyCode::Q]),
        TypingInput::new('R', vec![KeyCode::LeftShift, KeyCode::R]),
        TypingInput::new('S', vec![KeyCode::LeftShift, KeyCode::S]),
        TypingInput::new('T', vec![KeyCode::LeftShift, KeyCode::T]),
        TypingInput::new('U', vec![KeyCode::LeftShift, KeyCode::U]),
        TypingInput::new('V', vec![KeyCode::LeftShift, KeyCode::V]),
        TypingInput::new('W', vec![KeyCode::LeftShift, KeyCode::W]),
        TypingInput::new('X', vec![KeyCode::LeftShift, KeyCode::X]),
        TypingInput::new('Y', vec![KeyCode::LeftShift, KeyCode::Y]),
        TypingInput::new('Z', vec![KeyCode::LeftShift, KeyCode::Z]),

        // Numbers (no modifier)
        TypingInput::new('1', vec![KeyCode::Key1]),
        TypingInput::new('2', vec![KeyCode::Key2]),
        TypingInput::new('3', vec![KeyCode::Key3]),
        TypingInput::new('4', vec![KeyCode::Key4]),
        TypingInput::new('5', vec![KeyCode::Key5]),
        TypingInput::new('6', vec![KeyCode::Key6]),
        TypingInput::new('7', vec![KeyCode::Key7]),
        TypingInput::new('8', vec![KeyCode::Key8]),
        TypingInput::new('9', vec![KeyCode::Key9]),
        TypingInput::new('0', vec![KeyCode::Key0]),

        // Shifted numbers / symbols
        TypingInput::new('!', vec![KeyCode::LeftShift, KeyCode::Key1]),
        TypingInput::new('@', vec![KeyCode::LeftShift, KeyCode::Key2]),
        TypingInput::new('#', vec![KeyCode::LeftShift, KeyCode::Key3]),
        TypingInput::new('$', vec![KeyCode::LeftShift, KeyCode::Key4]),
        TypingInput::new('%', vec![KeyCode::LeftShift, KeyCode::Key5]),
        TypingInput::new('^', vec![KeyCode::LeftShift, KeyCode::Key6]),
        TypingInput::new('&', vec![KeyCode::LeftShift, KeyCode::Key7]),
        TypingInput::new('*', vec![KeyCode::LeftShift, KeyCode::Key8]),
        TypingInput::new('(', vec![KeyCode::LeftShift, KeyCode::Key9]),
        TypingInput::new(')', vec![KeyCode::LeftShift, KeyCode::Key0]),

        // Punctuation (no modifier)
        TypingInput::new(' ',  vec![KeyCode::Space]),
        TypingInput::new('\'', vec![KeyCode::Apostrophe]),
        TypingInput::new(',',  vec![KeyCode::Comma]),
        TypingInput::new('-',  vec![KeyCode::Minus]),
        TypingInput::new('.',  vec![KeyCode::Period]),
        TypingInput::new('/',  vec![KeyCode::Slash]),
        TypingInput::new(';',  vec![KeyCode::Semicolon]),
        TypingInput::new('=',  vec![KeyCode::Equal]),
        TypingInput::new('[',  vec![KeyCode::LeftBracket]),
        TypingInput::new('\\', vec![KeyCode::Backslash]),
        TypingInput::new(']',  vec![KeyCode::RightBracket]),
        TypingInput::new('`',  vec![KeyCode::GraveAccent]),

        // Shifted punctuation
        TypingInput::new('"', vec![KeyCode::LeftShift, KeyCode::Apostrophe]),
        TypingInput::new('<', vec![KeyCode::LeftShift, KeyCode::Comma]),
        TypingInput::new('_', vec![KeyCode::LeftShift, KeyCode::Minus]),
        TypingInput::new('>', vec![KeyCode::LeftShift, KeyCode::Period]),
        TypingInput::new('?', vec![KeyCode::LeftShift, KeyCode::Slash]),
        TypingInput::new(':', vec![KeyCode::LeftShift, KeyCode::Semicolon]),
        TypingInput::new('+', vec![KeyCode::LeftShift, KeyCode::Equal]),
        TypingInput::new('{', vec![KeyCode::LeftShift, KeyCode::LeftBracket]),
        TypingInput::new('|', vec![KeyCode::LeftShift, KeyCode::Backslash]),
        TypingInput::new('}', vec![KeyCode::LeftShift, KeyCode::RightBracket]),
        TypingInput::new('~', vec![KeyCode::LeftShift, KeyCode::GraveAccent]),
    ];

    for typing_input in typing_inputs {
        events.push(typing_input.get_event(&string_ref, &cursor_index_ref));
    }

    return events;
}


pub fn get_backspace_event(string_ref: &Rc<RefCell<String>>, cursor_index_ref: &Rc<RefCell<usize>>) -> Event {
    let mut backspace_event = StringEvent::RemoveCharWithRefIndex(string_ref.clone(), cursor_index_ref.clone()).wrap_into_event_vec();
    backspace_event.insert(1, UsizeEvent::ModUsize(cursor_index_ref.clone(), -1).wrap_into_event());
    return InputEvent::KeyDown(KeyCode::Backspace, backspace_event).wrap_into_event();
}

pub fn get_cursor_events(string_ref: &Rc<RefCell<String>>, cursor_index_ref: &Rc<RefCell<usize>>) -> Vec<Event> {
    let mut events = Vec::new();


    let left_arrow_event = UsizeEvent::ModUsize(cursor_index_ref.clone(), -1).wrap_into_event_vec();
    events.push(InputEvent::KeyDown(KeyCode::Left, left_arrow_event).wrap_into_event());

    let right_arrow_event = UsizeEvent::ModUsize(cursor_index_ref.clone(), 1).wrap_into_event_vec();
    events.push(InputEvent::KeyDown(KeyCode::Right, right_arrow_event).wrap_into_event());

    events.push(get_backspace_event(string_ref, cursor_index_ref));

    return events;
}

pub fn get_text_inputs(string_ref: &Rc<RefCell<String>>, cursor_index_ref: &Rc<RefCell<usize>>) -> Vec<Event> {
    let mut events = Vec::new();

    events.append(&mut get_cursor_events(string_ref, cursor_index_ref));
    events.append(&mut get_typing_events(string_ref, cursor_index_ref));
    

    return events;
}

use std::sync::Mutex;

static INDENTATION: Mutex<usize> = Mutex::new(0);


pub fn set_indent(indent: usize) {
    *INDENTATION.lock().unwrap() = indent;
}

pub fn log_indent() {
    *INDENTATION.lock().unwrap() += 1;
}

pub fn log_unindent() {
    let mut indent = INDENTATION.lock().unwrap();
    if *indent > 0 {
        *indent -= 1;
    }
}

pub fn log_mod_indent(value : f32) {
    let mut indent = INDENTATION.lock().unwrap();
    if *indent > 0 {
        let new_indent = ((*indent as f32) + value) as usize;
        *indent = new_indent;
    }
}

pub fn log_error(text: &str) {
    let indent = *INDENTATION.lock().unwrap();
    print!("{}", "  ".repeat(indent));
    println!("\x1b[31mX {}\x1b[0m", text);
}

pub fn log_init(text : &str)
{
    let indent = *INDENTATION.lock().unwrap();
    print!("{}", "  ".repeat(indent));
    if indent == 0 {
        println!("\x1b[32m+ {}\x1b[0m", text);
    }
    else {
        println!("\x1b[32mV {}\x1b[0m", text);
    }

}

pub fn log_default(text : &str)
{
    let indent = *INDENTATION.lock().unwrap();
    print!("{}", "  ".repeat(indent));
    println!("- {}", text);
}

pub fn log_header(text : &str)
{
    set_indent(0);
    println!();
    println!();

    println!("\x1b[1;34m{}\x1b[0m", "~".repeat(text.len() + 4));
    println!("\x1b[1;34m| {} |\x1b[0m", text);
    println!("\x1b[1;34m{}\x1b[0m", "~".repeat(text.len() + 4));
}
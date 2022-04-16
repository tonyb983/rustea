use crossterm::event::{KeyCode, KeyEvent};

pub struct Input {
    buffer: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn on_key_event(&mut self, key_event: KeyEvent) {
        println!("{:?}", key_event);
        match key_event.code {
            KeyCode::Backspace => {
                self.buffer.pop();
            }
            KeyCode::Char(c) => self.buffer.push(c),
            _ => (),
        }
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }
}

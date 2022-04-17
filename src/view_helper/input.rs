use crossterm::event::{KeyCode, KeyEvent};

pub struct Input {
    buffer: String,
    pos: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            pos: 0,
        }
    }

    pub fn on_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Backspace => {
                if self.pos > 0 {
                    self.buffer.remove(self.pos - 1);
                    self.pos -= 1;
                }
            }
            KeyCode::Char(c) => {
                self.buffer.insert(self.pos, c);
                self.pos += 1;
            }
            KeyCode::Left => {
                if self.pos > 0 {
                    self.pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.pos < self.buffer.len() {
                    self.pos += 1;
                }
            }
            _ => (),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pos = 0;
    }

    pub fn buffer(&self) -> String {
        // return owned buffer so buffer can be read and cleared together
        self.buffer.to_owned()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

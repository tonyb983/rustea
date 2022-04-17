use crossterm::event::{KeyCode, KeyEvent};

/// A helper struct for creating a user input field.
///
/// It is very minimal and leaves all control of rendering to the user.
/// You are able to access the buffer and caret pos, and render the caret however you please.
pub struct Input {
    buffer: String,
    pos: usize,
}

impl Input {
    /// Simple contructor. Starts with an empty buffer.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            pos: 0,
        }
    }

    /// Recieves crossterm `KeyEvent`s and updates the buffer and caret position.
    ///
    /// It handles:
    /// * Backspace. Deletes one character back from the current pos, and steps the pos back.
    /// * Chars. Inserts the character at the current pos, and steps the pos forward.
    /// * Left. Steps the pos back if possible.
    /// * Right. Steps the pos forward if possible.
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

    /// Emptys the buffer and resets the pos.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pos = 0;
    }

    /// Returns the current buffer.
    pub fn buffer(&self) -> String {
        // return owned buffer so buffer can be read and cleared together
        self.buffer.to_owned()
    }

    /// Overwrites the current buffer with the given string,
    /// and sets the pos to the end of it.
    pub fn set_buffer(&mut self, buffer: String) {
        self.pos = buffer.len();
        self.buffer = buffer;
    }

    /// Returns the current position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets the current position.
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}

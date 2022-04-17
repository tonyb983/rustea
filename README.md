# RusTea

An easy-to-use TUI crate for Rust, based off of the Elm architecture.
This is a re-implementation of Go's [Tea](https://github.com/tj/go-tea), created by TJ Holowaychuk.

## Quickstart

```rust
use rustea::crossterm_event::{KeyCode, KeyEvent, KeyModifiers};
use rustea::{quit_cmd, App, Command, Message};

struct Model {
    last_key: Option<char>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(quit_cmd),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Char(c) => {
                    self.last_key = Some(c);
                    return None;
                }
                _ => unimplemented!(),
            }
        };

        None
    }

    fn view(&self) -> String {
        format!("Hello! You pressed: {:?}", self.last_key)
    }
}

fn main() {
    let model = Model { last_key: None };

    rustea::run(model).unwrap();
}


```
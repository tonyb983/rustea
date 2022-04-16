use rustea::crossterm::event::{KeyCode, KeyEvent};
use rustea::{quit_cmd, App, Command, Message};

struct Model {
    last_key: Option<char>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            match key_event.code {
                KeyCode::Char('q') => return Some(quit_cmd),
                KeyCode::Char(c) => {
                    self.last_key = Some(c);
                    return None;
                }
                _ => return None,
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

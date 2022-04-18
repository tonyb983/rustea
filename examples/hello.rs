use rustea::crossterm::event::{KeyCode, KeyEvent};
use rustea::{App, Command, Message};

struct Model {
    last_key: Option<String>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        rustea::quit_if_ctrl_c!(msg);
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            self.last_key = match key_event.code {
                KeyCode::Char(c) => Some(c.to_string()),
                _ => None,
            };
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

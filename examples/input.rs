use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rustea::component::input::Input;
use rustea::{quit_cmd, App, Command, Message};

struct Model {
    input: Input,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            match *key_event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                } => return Some(quit_cmd),
                _ => self.input.on_key_event(*key_event),
            }
        };

        None
    }

    fn view(&self) -> String {
        self.input.buffer().to_owned()
    }
}

fn main() {
    let model = Model {
        input: Input::new(),
    };

    rustea::run(model).unwrap();
}

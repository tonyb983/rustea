use rustea::component::input::Input;
use rustea::crossterm_event::{KeyCode, KeyEvent, KeyModifiers};
use rustea::{quit_cmd, App, Command, Message};

struct Model {
    input: Input,
    name: Option<String>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(quit_cmd)),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Char(_) => {
                    self.input.on_key_event(*key_event);
                    return None;
                }
                KeyCode::Enter => {
                    self.name = Some(self.input.buffer().to_owned());
                    self.input.clear();
                    // return Some(quit_cmd);
                    return None;
                }
                _ => unimplemented!(),
            }
        };

        None
    }

    fn view(&self) -> String {
        let output = format!("Enter your name: {}", self.input.buffer());
        if let Some(name) = &self.name {
            format!("{}\nHello, {}!", output, name)
        } else {
            output
        }
    }
}

fn main() {
    let model = Model {
        input: Input::new(),
        name: None,
    };

    rustea::run(model).unwrap();
}

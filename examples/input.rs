use rustea::crossterm::event::{KeyCode, KeyEvent};
use rustea::view_helper::input::Input;
use rustea::{command, App, Command, Message};

struct Model {
    input: Input,
    name: Option<String>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if rustea::utils::is_ctrl_c(&key_event) {
                return Some(Box::new(command::quit));
            }

            match key_event.code {
                KeyCode::Enter => {
                    self.name = Some(self.input.buffer());
                    self.input.clear();
                    // return Some(quit_cmd);
                    return None;
                }
                _ => self.input.on_key_event(*key_event),
            }
        };

        None
    }

    fn view(&self) -> String {
        let prompt = "Enter your name: ";
        let output = format!(
            "{}{}\n{}^",
            prompt,
            self.input.buffer(),
            " ".repeat(prompt.len() + self.input.pos())
        );
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

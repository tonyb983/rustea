use rustea::crossterm::event::{KeyCode, KeyEvent};
use rustea::{command, App, Command, Message};

struct Model {
    last_key: Option<String>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            if rustea::utils::is_ctrl_c(key_event) {
                return Some(Box::new(command::quit));
            }

            if let KeyCode::Char('q') = key_event.code {
                panic!("boom")
            }

            self.last_key = Some(format!("{:?}", key_event.code));
            return None;
            // match key_event.code {
            //     KeyCode::Char(c) => {
            //         self.last_key = Some(c.to_string());
            //         return None;
            //     }
            //     _ => unimplemented!(),
            // }
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

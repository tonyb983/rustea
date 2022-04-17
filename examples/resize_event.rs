use rustea::{App, Command, Message, ResizeEvent};

struct Model {
    terminal_x: u16,
    terminal_y: u16,
    moved: bool,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(resize_event) = msg.downcast::<ResizeEvent>() {
            self.moved = true;
            self.terminal_x = resize_event.0;
            self.terminal_y = resize_event.1;
        }

        None
    }

    fn view(&self) -> String {
        if self.moved {
            format!(
                "Terminal size: (x: {}, y: {})",
                self.terminal_x, self.terminal_y
            )
        } else {
            format!("Resize terminal")
        }
    }
}

fn main() {
    let model = Model {
        terminal_x: 0,
        terminal_y: 0,
        moved: false,
    };

    rustea::run(model).unwrap();
}

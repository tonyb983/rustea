use rustea::{crossterm::event::MouseEvent, App, Command, Message};

struct Model {
    col: u16,
    row: u16,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(mouse_event) = msg.downcast::<MouseEvent>() {
            self.col = mouse_event.column;
            self.row = mouse_event.row;
        }

        None
    }

    fn view(&self) -> String {
        format!("Clicked row: {}, col: {}", self.col, self.row)
    }
}

fn main() {
    let model = Model { col: 0, row: 0 };

    rustea::run(model).unwrap();
}

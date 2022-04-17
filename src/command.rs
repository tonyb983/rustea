use crate::{Command, Message};

pub(crate) struct QuitMessage;
pub fn quit() -> Option<Message> {
    Some(Box::new(QuitMessage))
}

pub(crate) struct BatchMessage(pub Vec<Command>);
pub fn batch(cmds: Vec<Command>) -> Command {
    Box::new(|| Some(Box::new(BatchMessage(cmds))))
}

use crate::{Command, Message};

pub(crate) struct QuitMessage;

/// A built in command that quits the application.
pub fn quit() -> Option<Message> {
    Some(Box::new(QuitMessage))
}

pub(crate) struct BatchMessage(Vec<Command>);

#[allow(unused)]
impl BatchMessage {
    pub fn new(commands: Vec<Command>) -> Self {
        BatchMessage(commands)
    }

    pub fn commands(&self) -> &[Command] {
        &self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &Command> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Command> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Command> {
        self.0.into_iter()
    }
}

/// A built in command that combines multiple commands together.
///
/// These commands are executed in parallel, just like normal.
pub fn batch(cmds: Vec<Command>) -> Command {
    Box::new(|| Some(Box::new(BatchMessage(cmds))))
}

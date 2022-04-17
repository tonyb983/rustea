use crate::{Command, Message};

pub(crate) struct QuitMessage;

/// A built in command that quits the application.
pub fn quit() -> Option<Message> {
    Some(Box::new(QuitMessage))
}

pub(crate) struct BatchMessage(pub Vec<Command>);

/// A built in command that combines multiple commands together.
///
/// These commands are executed in parallel, just like normal.
pub fn batch(cmds: Vec<Command>) -> Command {
    Box::new(|| Some(Box::new(BatchMessage(cmds))))
}

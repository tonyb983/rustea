use crate::{utils::some_box, Command, Message};

pub struct QuitMessage;

/// A built in command that quits the application.
pub fn quit() -> Option<Message> {
    Some(Box::new(QuitMessage))
}

pub struct BatchMessage(Vec<Command>);

#[allow(unused)]
impl BatchMessage {
    pub fn new<I: IntoIterator<Item = Command>>(commands: I) -> Self {
        Self(commands.into_iter().collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for BatchMessage {
    type Item = Command;
    type IntoIter = std::vec::IntoIter<Command>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A built in command that combines multiple commands together.
///
/// These commands are executed in parallel, just like normal.
pub fn batch<I: IntoIterator<Item = Command>>(cmds: I) -> Command {
    let cmds = cmds.into_iter().collect::<Vec<Command>>();
    Box::new(|| some_box(BatchMessage(cmds)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::none_box;

    fn make_command_1() -> Command {
        Box::new(|| some_box(String::from("Command 1")))
    }

    fn make_command_2() -> Command {
        Box::new(none_box)
    }

    fn creates_commands() -> impl Iterator<Item = Command> {
        vec![
            make_command_1(),
            make_command_2(),
            make_command_1(),
            make_command_2(),
        ]
        .into_iter()
    }

    #[test]
    fn batch_inputs() {
        let _ = batch(vec![make_command_1(), make_command_2()]);
        let _ = BatchMessage::new(vec![make_command_1(), make_command_2()]);

        let _ = batch([make_command_1(), make_command_2()]);
        let _ = BatchMessage::new([make_command_1(), make_command_2()]);

        let hm = {
            let mut map = std::collections::HashMap::new();
            map.insert(1, make_command_1());
            map.insert(2, make_command_2());
            map
        };
        let _ = batch(hm.into_values());
        let hm = {
            let mut map = std::collections::HashMap::new();
            map.insert(1, make_command_1());
            map.insert(2, make_command_2());
            map
        };
        let _ = BatchMessage::new(hm.into_values());

        let _ = batch(creates_commands());
        let _ = BatchMessage::new(creates_commands());
    }

    #[test]
    fn batch_basic() {
        let batch = BatchMessage::new(creates_commands());
        assert!(!batch.is_empty());
        assert_eq!(batch.len(), 4);
        for cmd in batch {
            let _ = cmd();
        }
    }
}

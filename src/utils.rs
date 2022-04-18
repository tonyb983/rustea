use std::any::Any;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::Message;

pub fn some_box<T: Any + Send>(input: T) -> Option<Box<dyn Any + Send>> {
    Some(Box::new(input))
}

pub fn none_box() -> Option<Box<dyn Any + Send>> {
    None
}

pub fn is_ctrl_c(input: &KeyEvent) -> bool {
    matches!(
        *input,
        KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code: KeyCode::Char('c')
        },
    )
}

pub fn is_ctrl_c_msg(input: &Message) -> bool {
    if let Some(key_event) = input.downcast_ref::<KeyEvent>() {
        is_ctrl_c(key_event)
    } else {
        false
    }
}

/// Macro that can be called from within [`crate::App::init`] or [`crate::App::update`] to send
/// a [`crate::command::QuitMessage`] if the input is detected to be Ctrl+C.
///
/// ## Example
/// ```
/// # use rustea::{App, Command, Message};
/// struct MyApp {
///     message: String
/// }
///
/// impl MyApp {
///     fn new() -> Self {
///        Self { message: "Hello world!".to_string() }
///     }
/// }
///
/// impl rustea::App for MyApp {
///     fn update(&mut self, msg: rustea::Message) -> Option<rustea::Command> {
///         // Checks `msg` for Ctrl+C and returns rustea::command::QuitMessage if found.
///         rustea::quit_if_ctrl_c!(msg);
///         
///         todo!("Do actual work here.");
///
///         // ...or do nothing
///         None
///     }
///
///     fn view(&self) -> String { self.message.clone() }
/// }
/// # assert_eq!(MyApp::new().view(), "Hello world!");
/// ```
#[macro_export]
macro_rules! quit_if_ctrl_c {
    ($msg:expr) => {
        match (&$msg) {
            msg_ref => {
                if let Some(key_event) = msg_ref.downcast_ref::<crossterm::event::KeyEvent>() {
                    if matches!(
                        *key_event,
                        crossterm::event::KeyEvent {
                            modifiers: crossterm::event::KeyModifiers::CONTROL,
                            code: crossterm::event::KeyCode::Char('c')
                        },
                    ) {
                        return Some(Box::new($crate::command::quit));
                    }
                }
            }
        }
    };
}

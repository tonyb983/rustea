//! # Rustea
//!
//! `rustea` is a small crate for easily creating cross-platform TUI applications.
//! It is based off of the original [go-tea](https://github.com/tj/go-tea) created by TJ Holowaychuk.

pub mod view_helper;
pub extern crate crossterm;
pub mod command;

use std::{
    any::Any,
    io::{stdout, Result, Stdout},
    sync::mpsc::{self, Sender},
    thread,
};

use crossterm::{
    cursor,
    event::{read, Event},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

/// Any boxed type that may or may not contain data.
/// They are fed to your applications `update` method to tell it how and what to update.
///
/// Typically, you will use the `downcast_ref` method on your messages to determine the type of the message,
/// and extract the data from them if present.
///
/// # Example
///
/// ```
/// // the type of your message
/// struct HttpResponse(String);
///
/// // the boxed message itself
/// let http_response_message = Box::new(HttpResponse("Hello World".to_string()));
///
/// // determining the type of your message, and extracting the response
/// if let Some(res) = http_response_message.downcast_ref::<HttpResponse>() {
///     // do something with the response
///     // for example, setting it in the model to be rendered
///     model.response = Some(res);
/// }
/// ```
pub type Message = Box<dyn Any + Send>;

/// A boxed function or closure that performs computations and optionally dispatches messages.
/// All commands are processed in their own threads, so blocking commands are totally fine.
/// Frequently, data needs to be passed to commands. Since commands take no arguments,
/// a common solution to this is to build constructor functions.
///
/// # Example
///
/// ```
/// // a constructor function
/// fn make_request_command(url: &str) -> Command {
///     // it's okay to block since commands are multi threaded
///     let text_response = reqwest::blocking::get(url).unwrap().text().unwrap();
///     
///     // the command itself
///     Box::new(move || Some(Box::new(HttpResponse(text_response))))
/// }
pub type Command = Box<dyn FnOnce() -> Option<Message> + Send + 'static>;

/// The trait your model must implement in order to be `run`.
///
/// `init` is called once when the model is run for the first time, and optionally returns a `Command`.
/// There is a default implementation of `init` that returns `None`.
///
/// `update` is called every time your application recieves a `Message`.
/// You are allowed to mutate your model's state in this function.
/// It optionally returns a `Command`.
///
/// `view` is called after every `update` and is responsible for rendering the model.
/// It returns a `String` that is printed to the screen.
/// You are _not_ allowed to mutate the state of your application in the view, only render it.
///
/// For examples, check the `examples` directory.
pub trait App {
    fn init(&self) -> Option<Command> {
        None
    }

    fn update(&mut self, msg: Message) -> Option<Command>;
    fn view(&self) -> String;
}

/// Runs your application.
///
/// This will begin listening for keyboard events, and dispatching them to your application.
/// These keyboard events are handled by `crossterm`, and are fed into your `update` function as `Message`s.
/// You can access these keyboard events by simply downcasting them into a `crossterm::event::KeyEvent`.
///
/// `rustea` exports `crossterm`, so you can simply access it with `use rustea::crossterm`.
pub fn run(app: impl App) -> Result<()> {
    let mut app = app;
    let mut stdout = stdout();

    let (msg_tx, msg_rx) = mpsc::channel::<Message>();
    let msg_tx2 = msg_tx.clone();

    let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();
    let cmd_tx2 = cmd_tx.clone();

    thread::spawn(move || {
        loop {
            match read().unwrap() {
                Event::Key(event) => msg_tx.send(Box::new(event)).unwrap(),
                // TODO: handle these events
                _ => (),
            }
        }
    });

    thread::spawn(move || loop {
        let cmd = cmd_rx.recv().unwrap();

        let msg_tx2 = msg_tx2.clone();
        thread::spawn(move || {
            if let Some(msg) = cmd() {
                msg_tx2.send(msg).unwrap();
            }
        });
    });

    initialize(&mut stdout, &app, cmd_tx2)?;
    let mut prev = normalized_view(&app);
    execute!(stdout, Print(&prev))?;

    loop {
        let msg = msg_rx.recv().unwrap();
        if msg.is::<command::QuitMessage>() {
            break;
        } else if msg.is::<command::BatchMessage>() {
            let batch = msg.downcast::<command::BatchMessage>().unwrap();
            for cmd in batch.0 {
                cmd_tx.send(cmd).unwrap();
            }
        } else if let Some(cmd) = app.update(msg) {
            cmd_tx.send(cmd).unwrap();
        }

        let curr = normalized_view(&app);
        clear_lines(&mut stdout, prev.matches("\r\n").count())?;
        execute!(stdout, Print(&curr))?;
        prev = curr;
    }

    deinitialize(&mut stdout)
}

fn initialize(stdout: &mut Stdout, app: &impl App, cmd_tx: Sender<Command>) -> Result<()> {
    if let Some(cmd) = app.init() {
        cmd_tx.send(cmd).unwrap();
    }

    enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;

    Ok(())
}

fn normalized_view(app: &impl App) -> String {
    let view = app.view();
    let view = if !view.ends_with("\n") {
        view + "\n"
    } else {
        view
    };
    view.replace("\n", "\r\n")
}

fn clear_lines(stdout: &mut Stdout, count: usize) -> Result<()> {
    for _ in 0..count {
        execute!(
            stdout,
            cursor::MoveToPreviousLine(1),
            Clear(ClearType::CurrentLine)
        )?;
    }

    Ok(())
}

fn deinitialize(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, cursor::Show)?;
    disable_raw_mode()
}

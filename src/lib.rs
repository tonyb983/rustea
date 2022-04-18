//! # Rustea
//!
//! `rustea` is a small crate for easily creating cross-platform TUI applications.
//! It is based off of the original [go-tea](https://github.com/tj/go-tea) created by TJ Holowaychuk.
#![feature(associated_type_defaults, generic_associated_types)]

mod core;
pub use crate::core::*;
pub mod utils;
pub mod view_helper;

mod error;
pub use error::{Error, Result};

use std::{
    any::Any,
    io::{stdout, Stdout},
    sync::mpsc::{self, Sender},
    thread,
};

pub use crossterm;
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
///
/// TODO: Should this be a wrapper type instead of a type alias?
pub type Message = Box<dyn Any + Send>;

/// A boxed function or closure that performs computations and optionally dispatches messages.
/// All commands are processed in their own threads, so blocking commands are totally fine.
/// Frequently, data needs to be passed to commands. Since commands take no arguments,
/// a common solution to this is to build constructor functions.
///
/// # Example
///
/// ```
/// # use rustea::Command;
/// # #[derive(Debug, Clone, PartialEq, Eq)]
/// # struct HttpResponse(String);
/// # impl HttpResponse { pub fn get(&self) -> String { self.0.clone() } }
/// # struct Response(String);
/// # impl Response { pub fn text(&self) -> String { self.0.clone() } }
/// fn http_get(url: &str) -> Response {
///     // Send http get request using library or whatever.
///     // For example:
///     // reqwest::blocking::get(url).unwrap()
///     # Response("Hello World".to_string())
/// }
///
/// // a constructor function
/// fn make_request_command(url: &str) -> Command {
///     // it's okay to block since commands are multi threaded
///     let text_response = http_get(url).text();
///     
///     // the command itself
///     Box::new(move || Some(Box::new(HttpResponse(text_response))))
/// }
///
/// # let command = make_request_command("https://www.rust-lang.org");
/// # let response = command().unwrap().downcast::<HttpResponse>().unwrap();
/// # assert_eq!(response.get(), "Hello World");
/// ```
///
/// TODO: Should this be a wrapper type instead of a type alias?
pub type Command = Box<dyn FnOnce() -> Option<Message> + Send + 'static>;

/// Event representing a terminal resize (x, y).
/// Boxed as a message so it can be sent to the application.
pub struct ResizeEvent(pub u16, pub u16);

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
    type Output: std::fmt::Display = String;
    /// Perform any app-specific initialization here.
    /// TODO: Should this be a `&mut self`? App might need to change state during init
    fn init(&self) -> Option<Command> {
        None
    }

    /// Called every time a message is received. Main application logic should live here.
    fn update(&mut self, msg: Message) -> Option<Command>;

    /// Called after every update to retrieve the current visible state of your application.
    /// TODO: Does this have to be a String? Could we maybe return `impl Display` instead to give more flexibility?
    fn view(&self) -> Self::Output;

    /// Called when the application is closing, to perform any clean-up or resource deallocation
    /// your app requires.
    fn uninit(&mut self) {}
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

    let _read_handle = thread::spawn(move || loop {
        match read() {
            Ok(r) => match r {
                Event::Key(event) => msg_tx.send(Box::new(event))?,
                Event::Mouse(event) => msg_tx.send(Box::new(event))?,
                Event::Resize(x, y) => msg_tx.send(Box::new(ResizeEvent(x, y)))?,
            },
            Err(err) => return crate::Result::<()>::Err(err.into()),
        }
    });

    let _cmd_handle = thread::spawn(move || loop {
        let cmd = match cmd_rx.recv() {
            Ok(cmd) => cmd,
            Err(err) => return crate::Result::<()>::Err(err.into()),
        };
        let msg_tx2 = msg_tx2.clone();
        thread::spawn(move || match cmd() {
            Some(msg) => msg_tx2.send(msg),
            None => Ok(()),
        });
    });

    initialize(&mut stdout, &app, cmd_tx2)?;
    let mut prev = normalized_view(&app);
    execute!(stdout, Print(&prev))?;

    loop {
        let msg = msg_rx.recv()?;
        if msg.is::<core::QuitMessage>() {
            break;
        } else if msg.is::<core::BatchMessage>() {
            // TODO: This unwrap was probably safe since it is being checked with `Any::is` beforehand
            let batch = msg
                .downcast::<core::BatchMessage>()
                .map_err(|_| crate::Error::downcast("Unable to downcast msg (BatchMessage)"))?;
            for cmd in batch.into_iter() {
                cmd_tx.send(cmd)?;
            }
        } else if let Some(cmd) = app.update(msg) {
            cmd_tx.send(cmd)?;
        }

        let curr = normalized_view(&app);
        clear_lines(&mut stdout, prev.matches("\r\n").count())?;
        execute!(stdout, Print(&curr))?;
        prev = curr;
    }

    // Uncommenting these seems to cause the app to hang on exit, but otherwise how can
    //   we retrieve the error from the result.
    // if let Err(err) = read_handle.join() {
    //     eprintln!("Error joining read thread: {:?}", err);
    // }
    // if let Err(err) = cmd_handle.join() {
    //     eprintln!("Error joining command thread: {:?}", err);
    // }

    // This also causes the app to hang on exit.
    // if let Err(err) = read_handle.join() {
    //     std::panic::resume_unwind(err)
    // }
    // if let Err(err) = cmd_handle.join() {
    //     std::panic::resume_unwind(err)
    // }

    deinitialize(&mut stdout, &mut app)
}

/// Initializes the application, calling the `init` method of the given `app` and executing
/// any command that it may return, enabling raw mode through [`crossterm`], hiding the cursor, and
/// initializing mouse event capture.
fn initialize(stdout: &mut Stdout, app: &impl App, cmd_tx: Sender<Command>) -> Result<()> {
    if let Some(cmd) = app.init() {
        cmd_tx.send(cmd)?;
    }

    enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, crossterm::event::EnableMouseCapture)?;

    Ok(())
}

fn normalized_view(app: &impl App) -> String {
    let view = app.view().to_string();
    let view = if !view.ends_with('\n') {
        view + "\n"
    } else {
        view
    };
    #[cfg(windows)]
    {
        view.replace('\n', "\r\n")
    }
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

fn deinitialize(stdout: &mut Stdout, app: &mut impl App) -> Result<()> {
    app.uninit();
    execute!(stdout, crossterm::event::DisableMouseCapture)?;
    execute!(stdout, cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}

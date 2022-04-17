pub mod component;

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

pub use crossterm::event as crossterm_event;

pub trait App {
    fn init(&self) -> Option<Command> {
        None
    }

    fn update(&mut self, msg: Message) -> Option<Command>;
    fn view(&self) -> String;
}

pub type Message = Box<dyn Any + Send>;

pub type Command = Box<dyn FnOnce() -> Option<Message> + Send + 'static>;

struct Quit;
pub fn quit_cmd() -> Option<Message> {
    Some(Box::new(Quit))
}

struct Batch(Vec<Command>);
pub fn batch_cmd(cmds: Vec<Command>) -> Option<Message> {
    Some(Box::new(Batch(cmds)))
}

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
        if msg.is::<Quit>() {
            break;
        } else if msg.is::<Batch>() {
            // First check with is, then downcast so we can update without owning msg.
            let batch = msg.downcast::<Batch>().unwrap();
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

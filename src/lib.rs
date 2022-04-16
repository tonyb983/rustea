use std::{
    any::Any,
    io::{stdout, Result, Stdout},
    sync::mpsc::{self, Sender},
    thread,
};

use crossterm::{
    cursor, event, execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
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

pub type Command = fn() -> Option<Message>;

struct Quit;
pub fn quit_cmd() -> Option<Message> {
    Some(Box::new(Quit))
}

pub fn run(app: impl App) -> Result<()> {
    let mut app = app;
    let mut stdout = stdout();

    let (msg_tx, msg_rx) = mpsc::channel::<Message>();
    let msg_tx2 = msg_tx.clone();

    let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();
    let cmd_tx2 = cmd_tx.clone();

    thread::spawn(move || loop {
        let event = Box::new(event::read().unwrap());
        msg_tx.send(event).unwrap();
    });

    thread::spawn(move || {
        let cmd = cmd_rx.recv().unwrap();
        thread::spawn(move || {
            if let Some(msg) = cmd() {
                msg_tx2.send(msg).unwrap();
            }
        })
    });

    initialize(&mut stdout, &app, cmd_tx2)?;

    loop {
        let msg = msg_rx.recv().unwrap();
        if msg.is::<Quit>() {
            break;
        }

        if let Some(cmd) = app.update(msg) {
            cmd_tx.send(cmd).unwrap();
        }

        paint_screen(&mut stdout, &app.view())?;
    }

    deinitialize(&mut stdout)
}

fn initialize(stdout: &mut Stdout, app: &impl App, cmd_tx: Sender<Command>) -> Result<()> {
    if let Some(cmd) = app.init() {
        cmd_tx.send(cmd).unwrap();
    }

    enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;
    paint_screen(stdout, &app.view())
}

fn paint_screen(stdout: &mut Stdout, s: &str) -> Result<()> {
    for line in s.lines() {
        execute!(
            stdout,
            cursor::MoveToColumn(1),
            cursor::MoveUp(1),
            Clear(ClearType::CurrentLine),
            Print(line.to_owned() + "\r\n"),
        )?;
    }

    Ok(())
}

fn deinitialize(stdout: &mut Stdout) -> Result<()> {
    stdout.execute(cursor::Show)?;
    disable_raw_mode()
}

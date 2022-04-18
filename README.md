# Rustea

An easy-to-use TUI crate for Rust, based off of the Elm architecture.
This is a re-implementation of Go's [Tea](https://github.com/tj/go-tea), created by TJ Holowaychuk.

## Features

- Minimal and easy to use API.
- Growing collection of view helpers.
- Automatically multithreaded command processing.
- Cross-platform, thanks to `crossterm`.
- The praised Elm architecture.

## Installation and Docs

Install by putting `rustea = "0.1.0"` in your `Cargo.toml` dependencies.

Docs can be found on [docs.rs](https://docs.rs/rustea).

## Quickstart

An example demonstrating a website length checker, with batched asynchronous commands.

```rust
use crossterm::event::KeyModifiers;
use rustea::{
    command,
    crossterm::event::{KeyCode, KeyEvent},
    view_helper::input::Input,
    App, Command, Message,
};

struct Model {
    url_input: Input,
    website_lengths: Vec<usize>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                if let KeyCode::Char('c') = key_event.code {
                    return Some(Box::new(command::quit));
                }
            }

            match key_event.code {
                KeyCode::Enter => {
                    let url = self.url_input.buffer();
                    self.url_input.clear();

                    // make 3 requests to demonstrate command batching
                    let commands = vec![
                        make_request_command(url.clone()),
                        make_request_command(url.clone()),
                        make_request_command(url),
                    ];
                    return Some(command::batch(commands));
                }
                _ => self.url_input.on_key_event(*key_event),
            }
        } else if let Some(len) = msg.downcast_ref::<WebsiteLengthMessage>() {
            self.website_lengths.push(len.0);
        }

        None
    }

    fn view(&self) -> String {
        let mut out = format!(
            "Website URL (press enter when done): {}",
            self.url_input.buffer()
        );
        for (i, len) in self.website_lengths.iter().enumerate() {
            out.push_str(&format!("\nHit {} length: {}", i, len));
        }

        out
    }
}

struct WebsiteLengthMessage(usize);

fn make_request_command(url: String) -> Command {
    Box::new(move || {
        // It's okay to block since commands are multi threaded
        let website_len = reqwest::blocking::get(url).unwrap().bytes().unwrap().len();
        Some(Box::new(WebsiteLengthMessage(website_len)))
    })
}

fn main() {
    rustea::run(Model {
        url_input: Input::new(),
        website_lengths: Vec::new(),
    })
    .unwrap();
}

```

### More Examples

For more examples, see the examples directory.

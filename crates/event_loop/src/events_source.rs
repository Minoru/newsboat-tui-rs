//! Watcher for keypresses and signals.

use libc;
use signal_hook::iterator::Signals;
use std::{io, sync::mpsc, thread};
use termion::input::TermRead;

use ui::event::Event;

fn try_termion_key_to_ours(key: termion::event::Key) -> Option<ui::event::Key> {
    use ui::event::Key;
    match key {
        termion::event::Key::Backspace => Some(Key::Backspace),
        termion::event::Key::Left => Some(Key::Left),
        termion::event::Key::Right => Some(Key::Right),
        termion::event::Key::Up => Some(Key::Up),
        termion::event::Key::Down => Some(Key::Down),
        termion::event::Key::Home => Some(Key::Home),
        termion::event::Key::End => Some(Key::End),
        termion::event::Key::PageUp => Some(Key::PageUp),
        termion::event::Key::PageDown => Some(Key::PageDown),
        termion::event::Key::BackTab => None, // what is this key?
        termion::event::Key::Delete => Some(Key::Delete),
        termion::event::Key::Insert => Some(Key::Insert),
        termion::event::Key::F(f) => Some(Key::F(f)),
        termion::event::Key::Char(c) => Some(Key::Char(c)),
        termion::event::Key::Alt(c) => Some(Key::Alt(Box::new(Key::Char(c)))),
        termion::event::Key::Ctrl(c) => Some(Key::Ctrl(Box::new(Key::Char(c)))),
        termion::event::Key::Null => None, // what is this key?
        termion::event::Key::Esc => Some(Key::Esc),
        _ => None,
    }
}

/// Watcher for keypresses and signals.
pub struct EventsSource {
    /// Receive channel from which events can be read.
    rx: mpsc::Receiver<Event>,
}

impl EventsSource {
    /// Create a new events source.
    pub fn new() -> EventsSource {
        let (tx, rx) = mpsc::channel();

        // A thread that reads user input
        {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for event in stdin.keys() {
                    match event {
                        Ok(key) => {
                            if let Some(key) = try_termion_key_to_ours(key) {
                                if let Err(_) = tx.send(Event::Key(key)) {
                                    return;
                                }
                            }
                        }

                        Err(_) => {}
                    }
                }
            });
        }

        // A thread that watches out for signals
        {
            let tx = tx.clone();
            let mut signals = Signals::new(&[libc::SIGWINCH])
                .expect("Something went wrong while registering the handler");
            thread::spawn(move || {
                for signal in signals.forever() {
                    match signal {
                        libc::SIGWINCH => {
                            if let Err(_) = tx.send(Event::TerminalResized) {
                                return;
                            }
                        }

                        _ => {
                            // We didn't register for any other signals, so let's ignore them.
                        }
                    }
                }
            });
        }

        EventsSource { rx }
    }

    /// Get next event (blocking operation)
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

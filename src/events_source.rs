//! Watcher for keypresses and signals.

use libc;
use signal_hook::iterator::Signals;
use std::{io, sync::mpsc, thread};
use termion::input::TermRead;

/// Events that this watcher can report.
pub enum Event {
    /// User pressed a key.
    Key(termion::event::Key),

    /// Terminal changed size.
    ///
    /// This is SIGWINCH.
    TerminalResized,
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
                            if let Err(_) = tx.send(Event::Key(key)) {
                                return;
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
            let signals = Signals::new(&[libc::SIGWINCH])
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

    /// Get next key (blocking operation)
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

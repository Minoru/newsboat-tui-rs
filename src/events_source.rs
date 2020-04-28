//! Watcher for keypresses and signals.

use std::{io, sync::mpsc, thread};
use termion::input::TermRead;

/// Events that this watcher can report.
pub enum Event {
    Key(termion::event::Key),
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

        EventsSource { rx }
    }

    /// Get next key (blocking operation)
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

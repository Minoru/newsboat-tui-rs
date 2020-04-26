//! A thread that reads stdin and sends all received keys into a channel.

use std::{io, sync::mpsc, thread};
use termion::{event::Key, input::TermRead};

/// A thread that reads stdin and sends all received keys into a channel.
pub struct InputReader {
    /// Receive channel from which keys can be read
    rx: mpsc::Receiver<Key>,
}

impl InputReader {
    /// Create a new input reader
    pub fn new() -> InputReader {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            for event in stdin.keys() {
                match event {
                    Ok(key) => {
                        if let Err(_) = tx.send(key) {
                            return;
                        }
                    }

                    Err(_) => {}
                }
            }
        });

        InputReader { rx }
    }

    /// Get next key (blocking operation)
    pub fn next(&self) -> Result<Key, mpsc::RecvError> {
        self.rx.recv()
    }
}

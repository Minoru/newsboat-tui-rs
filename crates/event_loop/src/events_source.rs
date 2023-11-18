//! Watcher for keypresses and signals.

use std::{sync::mpsc, thread};

#[cfg(feature = "termion")]
use termion::input::TermRead;

#[cfg(feature = "crossterm")]
use crossterm::event;

use ui::event::Event;

#[cfg(feature = "termion")]
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

#[cfg(feature = "crossterm")]
fn try_crossterm_keycode_to_ours(keycode: event::KeyCode) -> Option<ui::event::Key> {
    use ui::event::Key;
    match keycode {
        event::KeyCode::Backspace => Some(Key::Backspace),
        event::KeyCode::Enter => Some(Key::Char('\n')),
        event::KeyCode::Left => Some(Key::Left),
        event::KeyCode::Right => Some(Key::Right),
        event::KeyCode::Up => Some(Key::Up),
        event::KeyCode::Down => Some(Key::Down),
        event::KeyCode::Home => Some(Key::Home),
        event::KeyCode::End => Some(Key::End),
        event::KeyCode::PageUp => Some(Key::PageUp),
        event::KeyCode::PageDown => Some(Key::PageDown),
        // TODO: test how Termion represents this and emulate it
        event::KeyCode::Tab => None,
        event::KeyCode::BackTab => None, // what is this key?
        event::KeyCode::Delete => Some(Key::Delete),
        event::KeyCode::Insert => Some(Key::Insert),
        event::KeyCode::F(f) => Some(Key::F(f)),
        event::KeyCode::Char(c) => Some(Key::Char(c)),
        event::KeyCode::Null => None, // what is this key?
        event::KeyCode::Esc => Some(Key::Esc),
        // Don't care about those
        event::KeyCode::CapsLock => None,
        event::KeyCode::ScrollLock => None,
        event::KeyCode::NumLock => None,
        event::KeyCode::PrintScreen => None,
        event::KeyCode::Pause => None,
        event::KeyCode::Menu => None,
        event::KeyCode::KeypadBegin => None,
        event::KeyCode::Media(_media_key_code) => None,
        event::KeyCode::Modifier(_modifier_key_code) => None,
    }
}

#[cfg(feature = "crossterm")]
fn try_crossterm_key_to_ours(key: event::KeyEvent) -> Option<ui::event::Key> {
    use ui::event::Key;
    try_crossterm_keycode_to_ours(key.code).map(|keycode| {
        let mut result = keycode;
        if key.modifiers.contains(event::KeyModifiers::SHIFT) {
            if let Key::Char(c) = result {
                // FIXME: our abstraction enforces one key == one codepoint, but converting to
                // uppercase can yield multiple ones. Can this be improved to avoid `expect()`?
                result = Key::Char(
                    c.to_uppercase()
                        .next()
                        .expect("Expected at least one char when uppercasing"),
                );
            }
        }
        result
    })
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
        #[cfg(feature = "termion")]
        {
            use std::io;

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

        #[cfg(feature = "crossterm")]
        {
            let tx = tx.clone();
            thread::spawn(move || loop {
                match event::read() {
                    Ok(event::Event::Key(key)) => {
                        if let Some(key) = try_crossterm_key_to_ours(key) {
                            if let Err(_) = tx.send(Event::Key(key)) {
                                return;
                            }
                        }
                    }

                    Ok(event::Event::Resize(_, _)) => {
                        if let Err(_) = tx.send(Event::TerminalResized) {
                            return;
                        }
                    }

                    Ok(event::Event::FocusGained) => {}
                    Ok(event::Event::FocusLost) => {}
                    Ok(event::Event::Mouse(_)) => {}
                    Ok(event::Event::Paste(_)) => {}

                    Err(_) => {}
                }
            });
        }

        // A thread that watches out for signals
        #[cfg(feature = "termion")]
        {
            use signal_hook::iterator::Signals;

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

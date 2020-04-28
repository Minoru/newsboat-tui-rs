//! A trait that each "formaction" (dialog) implements.

use termion::event::Key;
use tui::{backend::Backend, terminal::Frame};

/// A trait that each "formaction" (dialog) implements.
pub trait FormAction<B: Backend> {
    /// Handle `key` pressed by the user.
    fn handle_key(&mut self, key: Key);

    /// Draw this formaction onto the `frame`.
    fn draw(&mut self, frame: &mut Frame<B>);
}

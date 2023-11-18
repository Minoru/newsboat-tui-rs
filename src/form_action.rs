//! A trait that each "formaction" (dialog) implements.

use ratatui::{backend::Backend, terminal::Frame};
use termion::event::Key;

use crate::app::App;

/// A trait that each "formaction" (dialog) implements.
pub trait FormAction<B: Backend> {
    /// Handle `key` pressed by the user, possibly changing something within the `app` in the
    /// process.
    fn handle_key(&mut self, key: Key, app: &mut App<B>);

    /// Draw this formaction onto the `frame`.
    fn draw(&mut self, frame: &mut Frame);
}

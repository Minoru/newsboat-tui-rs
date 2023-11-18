//! A trait that each "formaction" (dialog) implements.

use ratatui::{backend::Backend, terminal::Frame};

use crate::app::App;
use crate::event::Key;

/// A trait that each "formaction" (dialog) implements.
pub trait FormAction<B: Backend> {
    /// Handle `key` pressed by the user, possibly changing something within the `app` in the
    /// process.
    fn handle_key(&mut self, key: Key, app: &mut App<B>);

    /// Draw this formaction onto the `frame`.
    fn draw(&mut self, frame: &mut Frame);
}

//! State and the behaviour of the application.

use termion::event::Key;
use tui::{backend::Backend, terminal::Frame};

use crate::feed_list::FeedList;

/// State of our application.
pub struct App {
    /// Should we quit on the next iteration of the event loop?
    pub should_quit: bool,

    /// Feedlist.
    pub feeds: FeedList,
}

impl App {
    /// Create new, empty app.
    pub fn new() -> App {
        App {
            should_quit: false,
            feeds: FeedList::new(),
        }
    }

    /// Handle key `key` pressed by the user.
    pub fn handle_key(&mut self, key: Key) {
        match key {
            Key::Char(c) if c == 'q' => self.should_quit = true,

            other => self.feeds.handle_key(other),
        }
    }

    /// Draw the app to the screen `frame`.
    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        self.feeds.draw(frame);
    }
}

//! State and the behaviour of the application.

use termion::event::Key;
use tui::{backend::Backend, terminal::Frame};

use crate::feed_list;
use crate::stateful_list::StatefulList;

/// State of our application.
pub struct App {
    /// Should we quit on the next iteration of the event loop?
    pub should_quit: bool,

    /// Feedlist.
    pub feeds: StatefulList,
}

impl App {
    /// Create new, empty app.
    pub fn new() -> App {
        App {
            should_quit: false,
            feeds: StatefulList::new(),
        }
    }

    /// Handle key `key` pressed by the user.
    pub fn handle_key(&mut self, key: Key) {
        match key {
            Key::Char(c) if c == 'q' => self.should_quit = true,

            Key::Up => self.feeds.previous(),

            Key::Down => self.feeds.next(),

            _ => {}
        }
    }

    /// Draw the app to the screen `frame`.
    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        feed_list::draw(frame, self);
    }
}

impl Default for App {
    fn default() -> App {
        let mut app = App::new();

        app.feeds.items = vec![
            "   1    (14/532) Planet Debian".to_string(),
            "   2       (0/1) Интересное на ДОУ".to_string(),
            "   3 N (23/4558) Fabio Franchino’s blog".to_string(),
            "   4      (0/13) @prometheusmooc on Twitter".to_string(),
            "   5    (12/482) /dev/lawyer".to_string(),
            "   6 N   (3/148) non-O(n) musings".to_string(),
        ];
        app.feeds.state.select(Some(0));

        app
    }
}

//! State and the behaviour of the application.

use crate::statefullist::StatefulList;

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

    /// Handle key `c` pressed by the user.
    pub fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
        }
    }
}

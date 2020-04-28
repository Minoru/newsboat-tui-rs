//! State and the behaviour of the application.

use termion::event::Key;
use tui::{backend::Backend, terminal::Frame};

use crate::feed_list::FeedList;
use crate::form_action::FormAction;
use crate::item_list::ItemList;

/// State of our application.
pub struct App<B: Backend> {
    /// Should we quit on the next iteration of the event loop?
    pub should_quit: bool,

    /// Are we in feedlist? If not, we're in the itemlist.
    pub in_feedlist: bool,

    /// Feedlist.
    pub feeds: Box<dyn FormAction<B>>,

    /// Itemlist.
    pub items: Box<dyn FormAction<B>>,
}

impl<B: Backend> App<B> {
    /// Create new, empty app.
    pub fn new() -> App<B> {
        App {
            should_quit: false,
            in_feedlist: true,
            feeds: Box::new(FeedList::new()),
            items: Box::new(ItemList::new()),
        }
    }

    /// Handle key `key` pressed by the user.
    pub fn handle_key(&mut self, key: Key) {
        match key {
            Key::Char(c) => match c {
                'q' => self.should_quit = true,

                'x' => self.in_feedlist = !self.in_feedlist,

                _ => {}
            },

            other => {
                if self.in_feedlist {
                    self.feeds.handle_key(other);
                } else {
                    self.items.handle_key(other);
                }
            }
        }
    }

    /// Draw the app to the screen `frame`.
    pub fn draw(&mut self, frame: &mut Frame<B>) {
        if self.in_feedlist {
            self.feeds.draw(frame);
        } else {
            self.items.draw(frame);
        }
    }
}

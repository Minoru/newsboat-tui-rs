//! State and the behaviour of the application.

use std::{cell::RefCell, rc::Rc};
use termion::event::Key;
use tui::{backend::Backend, terminal::Frame};

use crate::feed_list::FeedList;
use crate::form_action::FormAction;

/// State of our application.
pub struct App<B: Backend> {
    /// Should we quit on the next iteration of the event loop?
    pub should_quit: bool,

    /// Stack of open formactions.
    ///
    /// Should be non-empty. The topmost formaction (last one in the vector) is the current one;
    /// it's the one that is being drawn and processes all input.
    // Break down of the type, from the inside out:
    // - `dyn FormAction<B>` is basically a pointer to something that implements trait "FormAction"
    //   with backend "B";
    // - `RefCell<>` is an immutable thingy that lets us mutate what's inside. It implements borrow
    //   checking rules *at runtime*. If rules are broken, it will panic;
    // - `Rc` is a ref-counted pointer, like `std::shared_ptr` in C++.
    //
    // Combined, these things enable a neat trick: we can borrow App, pick a formaction from the
    // stack, *clone its Rc*, then let go of the borrow -- and still be able to call mutating
    // methods on the chosen formaction through the cloned Rc. Thanks to that trick, we can call
    // a method on a formaction that expects a mutable reference to App.
    //
    // Doing this naively would violate borrow checking rules, because we'd borrow App twice:
    // 1. once to get to the contents of formaction_stack;
    // 2. second time to form a parameter to the method.
    pub formaction_stack: Vec<Rc<RefCell<dyn FormAction<B>>>>,
}

impl<B: Backend> App<B> {
    /// Create new, empty app.
    pub fn new() -> App<B> {
        App {
            should_quit: false,
            formaction_stack: vec![Rc::new(RefCell::new(FeedList::new()))],
        }
    }

    /// Helper function for doing something with current formaction.
    ///
    /// The closure passed to this method takes two parameters:
    /// - first one is an Rc with current formaction;
    /// - second one is a mutable reference to Self, i.e. this method passes through the `self` on
    ///   which it was called. That `self` is no longer borrowed at this point, so you can safely
    ///   pass it as a parameter to something, or call more methods on it.
    fn with_current_formaction<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(Rc<RefCell<dyn FormAction<B>>>, &mut Self) -> T,
    {
        let formaction = match self.formaction_stack.last() {
            None => unreachable!("All formactions closed, but the app is still running"),

            Some(formaction) => formaction.clone(),
        };
        f(formaction, self)
    }

    /// Handle key `key` pressed by the user.
    pub fn handle_key(&mut self, key: Key) {
        self.with_current_formaction(|formaction, app| {
            formaction.borrow_mut().handle_key(key, app);
        });
    }

    /// Draw the app to the screen `frame`.
    pub fn draw(&mut self, frame: &mut Frame<B>) {
        self.with_current_formaction(|formaction, _app| {
            formaction.borrow_mut().draw(frame);
        });
    }
}

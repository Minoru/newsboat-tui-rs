//! State and the behaviour of the application.

use ratatui::{backend::Backend, terminal::Frame};
use std::{cell::RefCell, rc::Rc};

use crate::event::Key;
use crate::feed_list::FeedList;
use crate::form_action::FormAction;

/// State of our application.
pub struct App<B: Backend> {
    /// Should we quit on the next iteration of the event loop?
    pub should_quit: bool,

    /// List of currently open formactions.
    ///
    /// Should be non-empty. The "current" formaction (the one that gets rendered and processes all
    /// the input) is determined by `current_formaction_index`.
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
    formaction_stack: Vec<Rc<RefCell<dyn FormAction<B>>>>,

    /// The index of the formaction that gets rendered and processes all the input.
    current_formaction_index: usize,
}

impl<B: Backend> App<B> {
    /// Create new, empty app.
    pub fn new() -> App<B> {
        App {
            should_quit: false,
            formaction_stack: vec![Rc::new(RefCell::new(FeedList::new()))],
            current_formaction_index: 0,
        }
    }

    /// Helper function for doing something with the current formaction.
    ///
    /// The closure passed to this method takes two parameters:
    /// - first one is an Rc with current formaction;
    /// - second one is a mutable reference to Self, i.e. this method passes through the `self` on
    ///   which it was called. That `self` is no longer borrowed at this point, so you can safely
    ///   pass it as a parameter to something, or call more methods on it.
    ///
    /// # Panics
    ///
    /// Panics if the formaction stack is empty.
    fn with_current_formaction<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(Rc<RefCell<dyn FormAction<B>>>, &mut Self) -> T,
    {
        let formaction = self.formaction_stack[self.current_formaction_index].clone();
        f(formaction, self)
    }

    /// Handle key `key` pressed by the user.
    ///
    /// # Panics
    ///
    /// Panics if the formaction stack is empty.
    pub fn handle_key(&mut self, key: Key) {
        self.with_current_formaction(|formaction, app| {
            formaction.borrow_mut().handle_key(key, app);
        });
    }

    /// Draw the app to the screen `frame`.
    ///
    /// # Panics
    ///
    /// Panics if the formaction stack is empty.
    pub fn draw(&mut self, frame: &mut Frame) {
        self.with_current_formaction(|formaction, _app| {
            formaction.borrow_mut().draw(frame);
        });
    }

    /// Add given formaction to the top of the stack, i.e. make it the new current formaction.
    pub fn add_formaction(&mut self, formaction: Rc<RefCell<dyn FormAction<B>>>) {
        self.formaction_stack.push(formaction);
        self.current_formaction_index = self.formaction_stack.len().saturating_sub(1);
    }

    /// Remove current formaction from the stack.
    ///
    /// # Panics
    ///
    /// Panics if the formaction stack is empty.
    pub fn quit_current_formaction(&mut self) {
        let _ = self.formaction_stack.remove(self.current_formaction_index);
        self.current_formaction_index = self.formaction_stack.len().saturating_sub(1);
    }

    /// Switch to the next formaction in the list, wrapping to the first one if the end is reached.
    pub fn cycle_to_next_formaction(&mut self) {
        self.current_formaction_index =
            (self.current_formaction_index + 1) % self.formaction_stack.len();
    }

    /// Switch to the previous formaction in the list, wrapping to the last one if the beginning is
    /// reached.
    pub fn cycle_to_previous_formaction(&mut self) {
        if self.current_formaction_index == 0 {
            self.current_formaction_index = self.formaction_stack.len() - 1;
        } else {
            self.current_formaction_index = self.current_formaction_index - 1;
        }
    }
}

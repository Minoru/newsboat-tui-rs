//! Events (e.g. keypresses, terminal resize etc.) that this UI can handle.

/// Supported keys.
pub enum Key {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(Box<Key>),
    Ctrl(Box<Key>),
    Esc,
}

/// Kinds of events.
pub enum Event {
    /// User pressed a key.
    Key(Key),

    /// Terminal changed size.
    ///
    /// This is SIGWINCH.
    TerminalResized,
}

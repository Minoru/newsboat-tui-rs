This is a mockup of [Newsboat][newsboat]'s UI, redone with [ratatui][ratatui].
The goal is to evaluate ratatui as a replacement for [STFL][stfl] (see
[newsboat/newsboat#232][issue-232]).

NOTES in the root of the repo contains, well, notes: things I learned during
that exercise that I'd like to remember.

This can be built against two different terminal backends:

- `crossterm`: `cargo run --features crossterm`
- `termion`: `cargo run --features termion`

[newsboat]: https://newsboat.org/
[ratatui]: https://crates.io/crates/ratatui
[stfl]: http://www.clifford.at/stfl/
[issue-232]: https://github.com/newsboat/newsboat/issues/232

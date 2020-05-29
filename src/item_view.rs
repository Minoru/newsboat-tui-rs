//! A dialog displaying an article.

use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Text},
    Frame,
};

use crate::app::App;
use crate::form_action::FormAction;

/// A dialog displaying an article.
pub struct ItemView {
    /// A list of strings comprising the article.
    ///
    /// Each string has to end with a newline character.
    text: Vec<String>,

    /// Number of lines to skip at the beginning of the article.
    scroll_offset: u16,
}

impl ItemView {
    /// Create an example item view (with "lorem ipsum").
    pub fn new() -> ItemView {
        ItemView {
            text: LIPSUM
                .iter()
                .map(|text: &&str| String::from(*text))
                .collect::<Vec<_>>(),
            scroll_offset: 0,
        }
    }
}

impl<B: Backend> FormAction<B> for ItemView {
    fn draw(&mut self, frame: &mut Frame<B>) {
        let layout = Layout::default()
            .constraints(
                [
                    Constraint::Length(1), // title
                    Constraint::Min(0),    // feedlist
                    Constraint::Length(1), // hints
                    Constraint::Length(1), // command line (TODO: implement)
                ]
                .as_ref(),
            )
            .split(frame.size());

        {
            let title = [Text::styled(
                "Newsboat 2.20 (ну, почти) - Article 'Lorem Ipsum' (0 unread, 0 total)",
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .modifier(Modifier::BOLD),
            )];
            let paragraph = Paragraph::new(title.iter()).style(Style::default().bg(Color::Blue));
            frame.render_widget(paragraph, layout[0]);
        }

        {
            let text = {
                let mut text = vec![
                    Text::raw("Feed: Example feed\n"),
                    Text::raw("Title: An interesting article\n"),
                    Text::raw("Link: https://example.com/an-interesting-article.html\n"),
                    Text::raw("Date: Mon, 02 Mar 2004 05:06:07 +0800\n"),
                    Text::raw("\n"),
                ];
                text.extend(self.text.iter().map(Text::raw));
                text
            };

            let paragraph = Paragraph::new(text.iter())
                // This is cheating. In real Newsboat, word wrapping is done beforehand, and UI lib
                // is passed a list of strings that are already cut to length. But this will do for
                // demonstration purposes.
                .wrap(true)
                .scroll(self.scroll_offset);
            frame.render_widget(paragraph, layout[1]);
        }

        {
            let hints = [Text::styled(
                "q:Quit UP:Scroll up DOWN:Scroll down",
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .modifier(Modifier::BOLD),
            )];
            let paragraph = Paragraph::new(hints.iter()).style(Style::default().bg(Color::Blue));
            frame.render_widget(paragraph, layout[2]);
        }
    }

    fn handle_key(&mut self, key: Key, app: &mut App<B>) {
        match key {
            Key::Char(c) if c == 'q' => {
                // The key got passed to us, which means we're on top of the stack. Thus, we're
                // sure this returns Some() with an Rc that holds us. We drop it, thus this dialog
                // is removed and cleaned up.
                let _ = app.formaction_stack.pop();
            }

            // This lets us scroll past the end of the article, but for a demo, I don't care.
            Key::Up => self.scroll_offset = self.scroll_offset.saturating_sub(1),
            Key::Down => self.scroll_offset = self.scroll_offset.saturating_add(1),

            _ => {}
        }
    }
}

const LIPSUM: [&str; 37] = [
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi non ante porttitor, commodo lorem vitae, cursus mauris. Mauris mattis, turpis id convallis posuere, erat ante pharetra velit, sed blandit enim augue in urna. Maecenas nisl risus, aliquam molestie semper quis, placerat sed diam. Etiam viverra leo accumsan, ornare urna ac, porta nisi. Mauris ante diam, sollicitudin maximus pharetra ut, consectetur vitae ex. Nam at euismod tortor. Etiam imperdiet malesuada scelerisque. Vestibulum egestas odio in sapien vehicula, mattis maximus nisl imperdiet. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Curabitur scelerisque, risus vitae hendrerit malesuada, sem dui pellentesque turpis, et placerat nibh arcu posuere magna.\n",
                "\n",
                "Phasellus ut nibh at urna pellentesque ultricies.\n",
                "\n",
                "Proin faucibus cursus libero quis semper. Nam vitae convallis sapien. Curabitur sollicitudin magna vitae felis finibus, nec tristique dui dignissim. Nam faucibus, velit eleifend molestie gravida, eros diam egestas tellus, sed vestibulum augue nisl vitae sem. Vestibulum at feugiat elit. Morbi vel dictum diam, id semper nulla. Etiam rhoncus enim eget hendrerit pulvinar. Morbi ornare malesuada volutpat. Suspendisse ut posuere enim. Nullam fringilla elit ut urna porttitor, at imperdiet tortor luctus. Phasellus congue felis sed velit imperdiet, sed mattis odio dignissim: https://newsboat.org/releases/2.19/docs/newsboat.html?parameter1=first_value&parameter2=second_long_value&third_parameter=something_else_entirely_but_still_very_long. Fusce eu ex dui. Ut non tortor non diam lacinia consectetur. Donec pretium quam non egestas imperdiet. Maecenas scelerisque nisi vitae efficitur egestas.\n",
                "\n",
                "Suspendisse pretium convallis orci, eget suscipit est dignissim in. Nulla facilisi. Ut pulvinar neque ut nisl maximus, a finibus tellus commodo. Vestibulum sit amet fringilla metus, vel rhoncus est. Aenean leo nunc, fringilla quis luctus sed, aliquam vitae nunc. Praesent egestas placerat metus at condimentum. Aliquam nec lectus lobortis leo laoreet dapibus quis sit amet magna. Etiam efficitur libero ac neque mollis, id auctor nisl venenatis. Curabitur non lobortis enim. Nunc convallis tellus nec diam varius, vitae euismod nibh elementum. Aenean sit amet mi in nunc malesuada vehicula quis id lacus. Mauris iaculis, quam id fermentum aliquam, enim sem consectetur augue, at pretium orci nisi et purus. Maecenas convallis eu nibh non feugiat. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque sit amet leo at sapien sodales convallis eu quis dui. Interdum et malesuada fames ac ante ipsum primis in faucibus.\n",
                "\n",
                "In mattis ex mauris, quis sodales leo sodales vitae. Nam et enim lobortis, lobortis turpis id, mollis metus. Vivamus a rutrum mauris. Mauris volutpat eros purus, venenatis tempus magna convallis sed. Maecenas non efficitur lorem, quis facilisis ex. Integer libero tellus, pulvinar eu convallis vestibulum, efficitur id neque. Maecenas ut lectus in arcu pharetra condimentum. Duis tincidunt scelerisque cursus. Nam faucibus cursus tincidunt. Nulla molestie mi nec elementum tincidunt. Sed sed laoreet risus. Fusce dignissim eleifend semper. Sed dictum posuere sapien in suscipit. Donec efficitur mauris vitae tempus consequat. Mauris vel urna non orci placerat molestie a ac nunc.\n",
                "\n",
                "Suspendisse pellentesque, quam eget posuere tempor, erat ipsum pulvinar leo, congue convallis ex augue lobortis libero. Curabitur justo nulla, fringilla id nibh nec, blandit auctor tortor. Maecenas vitae rutrum tellus, sed rutrum mi. Vivamus in elit id diam auctor eleifend at sit amet felis. Sed iaculis odio sit amet ligula commodo malesuada. In ligula libero, condimentum vitae pellentesque eget, rhoncus vestibulum felis. Donec vel nunc et velit gravida porta. Nam fermentum, ex eu interdum posuere, tortor ligula viverra dolor, ut porta ex quam sit amet mauris. Proin diam risus, fringilla a turpis sit amet, euismod rhoncus nulla. Aliquam sagittis mauris molestie justo dictum, et porta urna dictum.\n",
                "\n",
                "Etiam eu luctus metus, vitae pulvinar dui. Donec in mauris ultrices, rhoncus nisl nec, condimentum arcu. Maecenas massa metus, sollicitudin vitae pretium at, efficitur eget ante. Vivamus eget elementum nibh. In hendrerit metus quis sapien pharetra viverra. Cras ut neque sapien. Aenean porta lorem aliquet ex pulvinar tincidunt ac non nibh. Aenean efficitur elit in tempor scelerisque.\n",
                "\n",
                "Phasellus varius ex non leo tristique, in ultricies lectus sodales. Vivamus efficitur convallis tellus, sit amet volutpat lorem ultricies at. Morbi luctus facilisis quam, at fringilla est tristique vel.\n",
                "\n",
                "Nam vestibulum condimentum finibus. Etiam sapien magna, molestie in mattis non, tempor nec nunc. Nunc a facilisis eros. Aliquam pharetra pretium turpis ut vulputate. Mauris vel diam risus. Maecenas non hendrerit dolor, sit amet porta massa. Donec euismod sagittis diam vitae auctor. Duis dignissim molestie neque ac laoreet. Sed non viverra justo. Nulla vitae nisi turpis.\n",
                "\n",
                "Pellentesque tincidunt interdum purus id fringilla. Vestibulum eget purus velit. Mauris eu tellus vel diam ullamcorper tincidunt sit amet at lorem. Donec eleifend a libero porta vehicula. Duis maximus turpis sed arcu vehicula, sed vulputate nunc hendrerit. Suspendisse eleifend at ante a aliquet. Nullam interdum finibus nunc quis dapibus. Vestibulum ac ex vitae tortor volutpat vestibulum. Aliquam aliquam sollicitudin pharetra. Duis ultricies sit amet eros id auctor. Morbi sit amet egestas sapien, nec convallis elit. Cras euismod ut felis ac efficitur. Donec venenatis venenatis eros nec pulvinar. Etiam a cursus lorem.\n",
                "\n",
                "Pellentesque auctor fermentum nisi eleifend aliquam. Pellentesque placerat quis tellus ac eleifend. Aenean a ultrices elit. Mauris pretium commodo urna, id vulputate tellus vehicula et. Vivamus finibus id ex sed pellentesque. Nunc augue erat, viverra nec libero vitae, mollis ultrices tortor. Sed fermentum consequat quam, ut molestie massa tincidunt vel. Aenean imperdiet, velit a commodo maximus, ex metus commodo nunc, id dapibus nunc leo in tellus. Nulla ullamcorper magna ante, id laoreet dui vestibulum eget. Pellentesque eget tristique velit. Proin vitae enim vitae massa sagittis ullamcorper et sit amet tellus. Nunc felis nisl, egestas non ante nec, varius efficitur erat. Praesent nibh arcu, porta luctus scelerisque ut, consequat ut arcu.\n",
                "\n",
                "Pellentesque at lacus eu nisi facilisis molestie. Nunc consectetur eros sed nisi suscipit, vitae porta lorem cursus. Quisque mattis feugiat eros, non placerat augue fermentum a. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Nam arcu massa, tempor nec nibh eu, commodo vehicula mi. Duis eleifend neque urna, sit amet tristique leo tempor vel. Aliquam tellus libero, auctor vitae lorem nec, convallis finibus diam. Morbi varius nec ante venenatis volutpat. Proin at tellus nisi. Sed at sapien eget ligula auctor maximus.\n",
                "\n",
                "Nullam ac risus tellus. Maecenas vestibulum metus quis nibh bibendum, quis faucibus lectus tincidunt. Integer at bibendum mi. Praesent consequat, dolor eu tempor feugiat, nisl eros consequat neque, sed suscipit leo ex vitae massa. Vestibulum sollicitudin elit eget risus tincidunt, vitae elementum urna pulvinar. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aenean quis urna vel diam feugiat aliquet. Cras rhoncus, diam ac luctus bibendum, quam nibh iaculis velit, vel pulvinar urna velit condimentum arcu.\n",
                "\n",
                "Quisque eros est, lacinia id fermentum et, aliquam sit amet ipsum. Nullam euismod sollicitudin mi, at egestas nunc egestas ut. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Donec id imperdiet mauris. Nullam et sodales magna, sit amet feugiat purus. In hac habitasse platea dictumst. Nulla vehicula magna leo, eget posuere nibh efficitur vel.\n",
                "\n",
                "Aenean sed semper libero, a semper eros. Duis ultrices sollicitudin auctor. Sed eleifend ultricies lorem, vitae bibendum magna ullamcorper vel. Nam tempor ante quis purus pellentesque sollicitudin. Vestibulum vitae tortor in felis blandit sollicitudin sed non velit. Donec faucibus a nisl vitae ornare. Fusce rhoncus turpis lorem, ac molestie odio rutrum et. Fusce sed bibendum odio. Sed hendrerit nibh condimentum consequat lacinia.\n",
                "\n",
                "Morbi placerat pretium convallis. Quisque non vestibulum nulla, in pharetra metus. Etiam molestie orci sed justo convallis accumsan ac tempor mi. Proin posuere ullamcorper convallis. Duis et suscipit massa, in mollis ex. Maecenas bibendum, leo vel porta feugiat, nulla mi blandit nisl, vitae blandit augue neque non urna. Sed ex ex, porttitor finibus ipsum eget, aliquam blandit diam. Mauris bibendum nisi nec leo rutrum, eget imperdiet nulla auctor. Donec porttitor nunc felis, eu maximus elit scelerisque vel. Etiam ut sapien feugiat, rhoncus mi ac, porta tortor.\n",
                "\n",
                "Morbi sed neque a ipsum dapibus congue. Nullam dapibus massa a massa aliquet, id pulvinar odio efficitur. Ut luctus quam eget efficitur mattis. In hac habitasse platea dictumst. Praesent tempus ex non nisl feugiat, eget tempor ipsum imperdiet. Donec laoreet luctus metus, dignissim pretium tellus luctus quis. Nunc sed dignissim dolor. Aenean mollis placerat ligula. Phasellus vel velit eu ante tempus pharetra at sed turpis. Integer scelerisque purus quis nisl porttitor, sed dapibus eros vehicula. Vestibulum viverra risus ac aliquam commodo. In quis odio nisl. Mauris ut metus porttitor, suscipit tortor sed, gravida urna.\n",
                "\n",
                "Donec interdum urna facilisis nunc molestie, eu semper tortor rhoncus. Cras dapibus, massa vitae venenatis placerat, magna risus dapibus magna, lacinia pulvinar ipsum augue at lacus. Duis ultrices odio sed libero mollis, in luctus nisi dapibus. Morbi ut nibh vel nulla ultrices placerat ac ut est. Morbi id dui neque. Ut fringilla semper enim non posuere. Nulla facilisi. Nullam scelerisque neque vitae massa accumsan, in aliquet neque ultricies. Curabitur pretium feugiat tellus vitae dapibus. Mauris nec magna eu lacus tincidunt blandit.\n",
                "\n",
                "Vestibulum placerat metus turpis, ac egestas elit iaculis sodales. Proin sit amet mauris tincidunt, gravida ipsum ac, suscipit leo. Morbi placerat maximus luctus. In tempus congue enim ut tincidunt. Vivamus lacinia nisl sit amet sollicitudin rhoncus. Integer eu luctus nunc, vel vestibulum libero. Proin interdum, eros sed dignissim tincidunt, neque augue convallis risus, non sodales enim urna eu neque. Pellentesque faucibus, dui id eleifend molestie, nunc ante bibendum risus, pharetra pharetra urna sapien nec lorem. Pellentesque aliquet lacus in dui aliquam convallis. Mauris in quam non massa vehicula pharetra in et nunc.\n",
            ];

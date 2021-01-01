use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

struct App {
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Min(0), Constraint::Length(4)].as_ref())
                .split(f.size());

            let messages: Vec<ListItem> = app
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                    ListItem::new(content)
                })
                .collect();
            let messages =
                List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(messages, chunks[0]);
            let input = Block::default().title("input").borders(Borders::ALL);
            f.render_widget(input, chunks[1])
        })?;

        terminal.set_cursor(0, terminal.size()?.bottom() - 3)?;
        terminal.show_cursor()?;

        match rl.readline(">> ") {
            Err(_) => {
                break;
            }
            Ok(line) => {
                app.messages.push(line);
            }
        };
    }
    Ok(())
}

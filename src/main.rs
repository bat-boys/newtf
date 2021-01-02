use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    text::{Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

const TEST_LOG: &str = r###"
% Trying to connect to bat: ::1 2901.
% Connected to bat.


=======    =======  =====     === ======== === ==== ===    ===================
======= === ======  ======= =====  ======  === ==== === === ========^^========
======= ==== ==== == ====== ===== = ==== = === ==== === ==== =======oo========
======= ==== === ==== ===== ===== == == == === ==== === ==== ====/-    -\=====
=======    =====      ===== ===== ===  === === ==== === ==== ===/==\  /==\====
======= ==== === ==== ===== ===== ======== === ==== === ==== =======--========
======= ==== === ==== ===== ===== ======== === ==== === ==== =================
======= === ==== ==== ===== ===== ======== === ==== === === ==================
=======    ===== ==== ===== ===== ======== ====    ====    ===================
============================================================ Logo by: Ricochet

               Welcome to BatMUD, Online since 14th April 1990!
          Email: batry@bat.org about any problems concerning BatMUD

         The game is owned by B.A.T. ry, a self-funded organization. 

  1 - enter the game                    s - game status
  2 - visit the game                    w - who is playing at the moment
  3 - create a new character            q - quit

+---- - - - -                                                         +
|   Forgot your password? Retrieve it from http://www.bat.org/
    BatMUD runs on 2x Quad-core E5-2643 w/ 128GB mem (help hardware)  |
+       
    
What is your name: astrax

Recovering character.
   zkTccc  (road) Macedonia Regen Crystal.
   zWBccc   _______                       _             _        
   z*cGcc  (_______)                     | |           (_)        
   zASccc   _  _  _ _____  ____ _____  __| | ___  ____  _ _____  
   zgFccc  | ||_|| (____ |/ ___) ___ |/ _  |/ _ \|  _ \| (____ | 
           | |   | / ___ ( (___| ____( (_| | |_| | | | | / ___ | 
           |_|   |_\_____|\____)_____)\____|\___/|_| |_|_\_____|
"###;

struct App {
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            messages: vec![TEST_LOG.to_string()]
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

    let mut text = Text::from(TEST_LOG);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Min(0), Constraint::Length(4)].as_ref())
                .split(f.size());

            let lines_height = text.height() as u16 - chunks[0].bottom() + 2;
            let paragraph = Paragraph::new(text.clone())
                .block(Block::default().borders(Borders::ALL).title("Batmud"))
                .wrap(Wrap { trim: false })
                .scroll((lines_height, 0));
            f.render_widget(paragraph, chunks[0]);
        })?;

        terminal.set_cursor(0, terminal.size()?.bottom() - 4)?;
        terminal.show_cursor()?;

        match rl.readline(">> ") {
            Err(_) => {
                break;
            }
            Ok(line) => {
                text.extend(Text::raw(line));
            }
        };
    }
    Ok(())
}

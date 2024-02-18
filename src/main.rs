// use crossterm::{
//     event::{self, KeyCode, KeyEventKind},
//     terminal::{
//         self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
//     },
//     ExecutableCommand
// };

use std::fmt::format;

use crossterm::event::KeyCode;
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};




fn main() -> Result<(), Box<dyn std::error::Error>>{
    // stdout().execute(EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut counter = 0;

    loop {
        terminal.draw(|f|{
            f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => counter +=1,
                        crossterm::event::KeyCode::Char('k') => counter -=1,
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

use crossterm::{
    event::{self, KeyCode, KeyEventKind}, execute, terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    }
};

use anyhow::Result;
use std::fmt::format;

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};


struct App {
    counter: i64,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<()>{
    startup()?;

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

    shutdown()?;
    Ok(())
}

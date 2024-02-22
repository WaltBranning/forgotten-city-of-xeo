use std::io::{self, stdout, Error};
// mod event;
// use crate::event::*;
use crossterm::{
    execute,
    event::{
        self,
        Event,
        KeyCode,
        
    }, 
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }
};
use ratatui::{backend::CrosstermBackend, Terminal};
mod chatbuild;
use crate::chatbuild::*;
mod ui;
use crate::ui::ui;

fn main() -> Result<(), io::Error> {

    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let chat = chat_interface("Descripe in five words the Rust programming language.");
    loop {
        terminal.draw(|frame| ui(frame))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Char('e') => break,
                KeyCode::Char('q') => break,
                _=>{}
            }
        }
    }

    execute!(stderr, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

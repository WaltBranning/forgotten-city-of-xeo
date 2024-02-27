use std::{future, io::{self, stdout, Error}, task::Poll};
// mod event;
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
mod tui;
mod app;
use crate::app::*;
use crate::ui::ui;
use crate::tui::*;

#[tokio::main]
async fn main() -> Result<()> {

    let chat = chat_interface("Greet the adventure and welcome him to the 
    upcoming journey").await;
    // println!("{:?}", chat);

    let result = run().await;

    Ok(())
}

async fn run() -> Result<()> {
    let mut tui = tui::Tui::new()?
        .tick_rate(4.0)
        .frame_rate(30.0);
    tui.enter()?;

    let mut app = App { should_quit: false, text: Some(String::new()), current_screen: None, load_screen: format!("intro").into()};

    loop {
        tui.draw(|frame| ui(&app, frame))?;
        // print!("{:?}", app.text);

        if let Some(event) = tui.next().await {
            update(&mut app, event).await?;
        };
                
        if app.should_quit {
            break;
        }

    }


    tui.exit()?;
    Ok(())
}
use std::{
    collections::VecDeque,
    io::{self, 
        stdout, 
        Error}, 
    task::Poll};
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
use game::{AppData, GameSettings};
use ratatui::{backend::CrosstermBackend, Terminal};
mod chatbuild;
use crate::chatbuild::*;
mod ui;
mod tui;
mod app;
mod game;
use crate::app::*;
use crate::ui::ui;
use crate::tui::*;

#[tokio::main]
async fn main() -> Result<()> {

    let result = run().await;

    Ok(())
}

async fn run() -> Result<()> {
    let mut tui = tui::Tui::new()?
        .tick_rate(4.0)
        .frame_rate(80.0);
    
    tui.enter()?;
    // println!("Checking After Enter");
    let mut app = App { should_quit: false, text: Some(String::new()), text_buffer: None, current_screen: None, load_screen: format!("introduction").into(), control_text:String::new()};
    // app.set_load_screen(format!("token_input"));
    println!("{:?}", app);
    loop {
        tui.draw(|frame| ui(&app, frame))?;
        // print!("{:?}", app.text);
        if let Some(text) = app.text_buffer.as_mut()  {
            let new_val = text.pop_back();
            if new_val.is_some() {
                let old_val = app.text.clone().unwrap();
                let new_text = format!("{}{}",old_val,  &new_val.unwrap().to_string());
                app.text.replace(new_text);
            }
        }
        if let Some(event) = tui.next().await {
            update(&mut app, event).await?;
        };
                
        if app.should_quit {
            break;
        };

    }
    tui.exit()?;
    Ok(())
}
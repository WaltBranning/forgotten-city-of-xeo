use crate::tui::*;
use crossterm::{
    event::{
        KeyCode::Char,
    }
};

use crate::chatbuild::chat_interface;

pub type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;

pub struct App {
    pub should_quit: bool,
    pub text: Option<String>,
    pub current_screen: Option<String>,
    pub load_screen: Option<String>,
}

impl App {
    pub async fn get_location_text(&mut self) {
        if let Some(screen) = &self.load_screen {
            let text_data = chat_interface("Greet the adventure and welcome him to the upcoming journey").await;
            self.text.replace(text_data.choices[0].message.content.clone());
            // self.text.replace("Hello traveler!".into());
            self.load_screen.take();
        }
    }
}

pub async fn update(app: &mut App, event: Event) -> Result<()> {
    let load_screen = &app.load_screen;
    app.get_location_text().await;


    if let Event::Key(key) = event {
        match key.code {
            Char('q') => app.should_quit = true,
            _=>{}
        }
    }
    Ok(())
}
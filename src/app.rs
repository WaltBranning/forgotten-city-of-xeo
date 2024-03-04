use crate::{game::Settings, tui::*};
use color_eyre::eyre::OptionExt;
use crossterm::event::KeyCode::Char;
use ratatui::symbols::bar::Set;
use serde::{Deserialize, Serialize};
use crate::chatbuild::chat_interface;

pub type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub text: Option<String>,
    pub current_screen: Option<String>,
    pub load_screen: Option<String>,
    // pub settings: Settings,
}

impl App {
    pub async fn get_location_text(&mut self, location_name: String) -> Result<()> {
        let mut text = String::new();
        // let location: = Settings::location_data(self.load_screen.clone().ok_or_eyre("Unable to get location")?);
        if let Some(location) = Settings::location_data(location_name) {
            // println!("{:?}", location);
            
            let text_data = chat_interface(location.prompt.to_string()).await;
            
            match text_data {
                Ok(v) => text = v.choices[0].message.content.clone(),
                Err(e) => text = format!("Encountered Error: {}", e).to_string(),
            }
        };
        
        self.text.replace(text);
        // self.text.replace("Hello traveler!".into());
        self.load_screen.take();    
        Ok(())
    }

    pub fn set_load_screen(&mut self, location: String) -> Result<()> {
        self.load_screen.replace(location);
        Ok(())
    }
}



pub async fn update(app: &mut App, event: Event) -> Result<()> {
    let load_screen = &app.load_screen;
    
    if load_screen.is_some() {app.get_location_text(load_screen.clone().unwrap()).await?;}
    
    if let Event::Key(key) = event {
        match key.code {
            Char('q') => app.should_quit = true,
            _=>{}
        }
    }
    Ok(())
}


// #[derive(Serialize, Deserialize, Debug)]
// pub struct ChatSettings {
//     token: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Settings {
//     ChatToken
// }

// // #[derive(Serialize, Deserialize, Debug)]
// // pub struct Setting {
// //     pub chat_token: Option<String>,
// // }

// impl Settings {
//     pub fn get(&self) -> Option<Settings> {
//         println!("{:?} This is the value!!", &self);
//         match &self {
//             Settings::ChatToken => {
//                 // let token = GameData::read("settings");
//                 // println!("{:?}", token)
//             }
//         }
//         None
//     }
// }

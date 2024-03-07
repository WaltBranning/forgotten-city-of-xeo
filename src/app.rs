use crate::{game::Settings, tui::*};
use color_eyre::eyre::OptionExt;
use crossterm::{event::KeyCode::Char, style::Stylize};
use ratatui::symbols::bar::Set;
use serde::{Deserialize, Serialize};
use crate::chatbuild::chat_interface;
use crate::game::LocationCommand;
use std::collections::{HashMap, VecDeque};

pub type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub text: Option<String>,
    pub text_buffer: Option<VecDeque<char>>,
    pub current_screen: Option<String>,
    pub load_screen: Option<String>,
    pub commands: Option<Vec<LocationCommand>>,
    // pub settings: Settings,
}

impl App {
    pub async fn get_location_data(&mut self, location_name: String) -> Result<()> {
        let mut text = String::new();
    
        if let Some(location) = Settings::location_data(location_name) {
            
            let prompt = location.prompt.to_string();
            let details = location.description.to_string();
            let text_data = chat_interface(vec![prompt,details].join(" ")).await;

            self.commands.replace(location.commands);

            match text_data {
                Ok(v) => {
                    text = v.choices[0].message.content.clone();
                },
                Err(e) => text = format!("Encountered Error: {}", e).to_string(),
            }
            
            let reversed_text: VecDeque<char> = text.chars().rev().collect();
                self.text_buffer.replace(reversed_text);
                self.load_screen.take();    
        };
        Ok(())
    }

    pub fn set_load_screen(&mut self, location: String) -> Result<()> {
        self.load_screen.replace(location);
        Ok(())
    }
}



pub async fn update(app: &mut App, event: Event) -> Result<()> {
    let load_screen = &app.load_screen;
    
    if load_screen.is_some() {app.get_location_data(load_screen.clone().unwrap()).await?;}
    
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

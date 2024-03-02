use crate::{game::Settings, tui::*};
use crossterm::event::KeyCode::Char;
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
    pub async fn get_location_text(&mut self) {
        // println!("Now Getting Location Text");
        if let Some(screen) = &self.load_screen {
            let text_data = chat_interface("Greet the adventure and welcome him to the upcoming journey").await;
            self.text.replace(text_data.choices[0].message.content.clone());
            // self.text.replace("Hello traveler!".into());
            self.load_screen.take();
        }
    }

    pub fn set_load_screen(&mut self, location: String) -> Result<()> {
        self.load_screen.replace(location);
        Ok(())
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

use crate::{game::{Settings, GameSettings}, tui::*, chatbuild::{chat_interface, ChatConfig}};
use crossterm::event::KeyCode::{self, Char};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use crate::game::LocationCommand;
use std::collections::VecDeque;

pub type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Clone)]
pub struct App {
    pub should_quit: bool,
    pub text: Option<String>,
    pub text_buffer: Option<VecDeque<char>>,
    pub location_label: Option<String>,
    pub current_location: Option<String>,
    pub load_location: Option<String>,
    pub commands: Option<Vec<LocationCommand>>,
    pub control_state: ControlState,
    // pub settings: Settings,
}

impl App {
    pub async fn get_location_data(&mut self, location_name: String) -> Result<()> {
        let mut text = String::new();
    
        if let Some(location) = Settings::location_data(location_name) {
            let prompt = location.prompt.to_string();
            let details = location.description.to_string();
            let text_data = chat_interface(vec![prompt,details].join(" ")).await;

            self.commands.replace(location.commands.clone());
            // self.set_commands(location.commands);

            match text_data {
                Ok(v) => {
                    text = v.choices[0].message.content.clone();
                },
                Err(e) => text = format!("Encountered Error: {}", e).to_string(),
            }
            self.refresh_text(text)?;
            self.location_label.replace(location.name);
            self.control_state.state.select(Some(0));
               
        };
        Ok(())
    }

    // Grab the command at the index of selected item in control list and execute.
    pub async fn execute_command(&mut self) {
        if let Some(commands) = &self.commands.clone() {
            let active_command =  &commands[self.control_state.state.selected().unwrap_or(0)];

            match active_command.command_type.as_str() {
                "move" => {
                    self.set_load_location(active_command.location.clone());
                },
                "action"=> {
                    // println!("Called action");
                    self.get_action_text(active_command.prompt.clone()).await;
                },
                _ => {}
            }
            // println!("{:?}",active_command);
        }
    }

    pub async fn get_action_text(&mut self, action: String) -> Result<()>{
        let mut text = String::new();
        // println!("Fetching Action Text");
        // println!("{}", action);
        let text_data = chat_interface(action).await;
        
        match text_data {
            Ok(v) => {
                text = v.choices[0].message.content.clone();
            },
            Err(e) => text = format!("Encountered Error: {}", e).to_string(),
        }
        self.refresh_text(text)?;
        Ok(())
    }

    pub fn refresh_text(&mut self, text:String) -> Result<()> {
        // println!("Refreshing Text");        
        let reversed_text: VecDeque<char> = text.chars().rev().collect();
        self.text_buffer.replace(reversed_text);
        self.text.replace("".to_string());
        self.load_location.take(); 
        Ok(())
    }

    pub fn set_load_location(&mut self, location: String) -> Result<()> {
        self.load_location.replace(location);
        Ok(())
    }

    pub async fn update(&mut self, event: Event) -> Result<()> {
        let load_location = &self.load_location;
        
        if load_location.is_some() {
            self.get_location_data(load_location.clone().unwrap()).await?;
            // app.control_state.state.select(Some(0));
        }
        
        if let Event::Key(key) = event {
            match key.code {
                Char('q') | KeyCode::Esc => self.should_quit = true,
                Char('s') | KeyCode::Down => self.control_state.next(self.commands.clone()),
                Char('w') | KeyCode::Up => self.control_state.previous(self.commands.clone()),
                KeyCode::Enter => self.execute_command().await,
                _=>{}
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppData {
     pub chat_config: ChatConfig,
}

impl AppData {
    pub fn chat_config () -> ChatConfig {
        // println!("Now Access chat_config");
        let file = GameSettings::AppSettings.read("gamesettings");
        file.app.chat_config
    }
}

#[derive(Debug, Clone)]
pub struct ControlState {
    pub state: ListState,
    pub commands: Option<Vec<LocationCommand>>, //Commands is holding no values right now.
    pub current_selected: Option<usize>,
}

// I am not crazy about handling the state this way.  
// It is slightly inefficeint but I don't have to rewrite previous code. 
// So it stays for now, LOL.
// This maybe a future refinement.

impl ControlState {
    fn next(&mut self, commands_list:  Option<Vec<LocationCommand>>){        
        if let Some(commands) = commands_list {
            let i = match self.state.selected(){
                Some(i) => {
                        if i >= commands.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    } 
                None => self.current_selected.unwrap_or(0),
            };
            self.state.select(Some(i));
        };
    }
    fn previous(&mut self, commands_list:  Option<Vec<LocationCommand>>){
            if let Some(commands) = commands_list {
                let i = match self.state.selected(){
                    Some(i) => {
                            if i == 0 {
                                commands.len() - 1
                            } else {
                                i - 1
                            }
                        } 
                    None => self.current_selected.unwrap_or(0),
                };
                self.state.select(Some(i));
                };
            }
}



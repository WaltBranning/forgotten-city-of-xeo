use serde::{Deserialize, Serialize};
use crate::app::*;
use std::{
    collections::HashMap, env::current_dir, fs, path::Path
};

#[derive(Serialize, Deserialize, Debug)]
pub enum GameSettings {
    WorldSettings,
    AppSettings,
    UserSettings,
}

impl GameSettings {
    pub fn read(&self, file: &str) -> Settings {
        // println!("Now Reading GameSetting File");
        let input_file = format!("data/{file}.json");
        let file_path = current_dir().unwrap().join(Path::new(&input_file));
        let contents = fs::read_to_string(file_path)
            .expect("Couldn't find file.");
        let value: Settings = serde_json::from_str(contents.as_str())
            .expect("Unable to load JSON file");
        value
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub locations: Vec<Location>,
    pub world: WorldData,
    pub app: AppData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldData {
    pub speaker_role:  String,
    pub introduction: String,
    pub description: String,
    pub history: String,
    pub system: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationCommand {
    pub command_type: String,
    pub label: String,
    pub location: String,
    pub prompt: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prompt: String,
    pub commands: Vec<LocationCommand>,
}

impl Settings {

    pub fn world_data() -> WorldData {
        // println!("Now Reading WorlData");
        let file = GameSettings::WorldSettings.read("gamesettings");
        file.world
    }

    pub fn location_data(id: String) -> Option<Location> {
        // println!("Now Reading Locations");
        let file = GameSettings::WorldSettings.read("gamesettings");
        for location in file.locations {
            if location.id == id {
                return Some(location)
            } 
        }
        println!("None found");
        None
    }

}
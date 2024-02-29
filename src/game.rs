use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
    env::current_dir
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub locations: Vec<Location>,
    pub world: WorldData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldData {
    pub speaker_role:  String,
    pub introduction: String,
    pub description: String,
    pub history: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prompt: String,
}

impl GameData {
    fn read() -> Self{
        let file_path = current_dir().unwrap().join(Path::new("data/prompts.json"));
        let contents = fs::read_to_string(file_path)
            .expect("Couldn't find file.");
        let value: Self = serde_json::from_str(contents.as_str())
            .expect("Unable to load JSON file");
        value
    }

    pub fn world_data() -> WorldData{
        let file = Self::read();
        file.world
    }

    pub fn location_data(id: String) -> Option<Location> {
        let file = Self::read();
        for location in file.locations {
            if location.id == id {
                return Some(location)
            } 
        }
        None
    }

}
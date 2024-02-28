use chat_gpt_lib_rs::{ChatGPTClient, ChatInput, Message, Model, Role, ChatResponse};
use std::{
    collections::HashMap, fs, io::Error};
use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};
// use tokio;
// use color_eyre::eyre::Ok;


const API_KEY: &str = "";
const BASE_URL: &str = "https://api.openai.com";


pub async fn chat_interface(request: &str) -> ChatResponse {
    let client:ChatGPTClient = ChatGPTClient::new(API_KEY, BASE_URL);
    let role_description = GameData::world_data().speaker_role;
    println!("{:?}", role_description);
    let chat_input = ChatInput {
        model: Model::Gpt3_5Turbo,
        messages: vec![
            Message {
                role: Role::System,
                content: role_description,
            },
            Message {
                role: Role::User,
                content: request.to_string(),
            }
        ],
        ..Default::default()
    };
    let response = client.chat(chat_input).await.unwrap();
    response
}


#[derive(Serialize, Deserialize, Debug)]
struct GameData {
    locations: Vec<Locations>,
    world: WorldData,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorldData {
    speaker_role:  String,
    introduction: String,
    description: String,
    history: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Locations {
    id: String,
    name: String,
    description: String,
    prompt: String,
}

impl GameData {
    pub fn read() -> Self{
        let file_path = "/home/walter/Projects/Rust/forgotten-city-of-xeo/forgotten-city-of-xeo/src/promts.json".to_owned();
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

}
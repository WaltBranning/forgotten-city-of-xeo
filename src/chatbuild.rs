use chat_gpt_lib_rs::{
    ChatGPTClient,
    ChatInput, 
    Message, 
    Model, 
    Role, 
    ChatResponse
};
use std::result::Result;
use serde::{Deserialize, Serialize};
use color_eyre::eyre::Error;

use crate::app::*;
use crate::game::Settings;

const BASE_URL: &str = "https://api.openai.com";

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatConfig {
    pub token: String,
    pub model: String,
}

pub async fn chat_interface(request: String, ) -> Result<ChatResponse, Error> {
    let chat_config = AppData::chat_config();
    let token: String = chat_config.token;
    let model:String = chat_config.model;
    let client:ChatGPTClient = ChatGPTClient::new(&token, BASE_URL);
    let role_description = Settings::world_data().speaker_role;
    // let introducton = Settings::world_data().introduction;
    let description = Settings::world_data().description;
    let history = Settings::world_data().description;
    let system = Settings::world_data().system;

    // let location = Settings::location_data("gardens".to_string());
    // println!("{:?}",location);
    let gpt_model = |model:String|{
        match model.as_str() {
            "Gpt3_5Turbo" => Model::Gpt3_5Turbo,
            "Gpt4_Turbo" => Model::Gpt_4Turbo,
            "Gpt4" => Model::Gpt_4,
            "Gpt4_32kTurbo" => Model::Gpt_4_32k,
            _=>Model::Gpt3_5Turbo
        }
    };

    let chat_input = ChatInput {
        model: gpt_model(model),
        // model: Model::Gpt_4Turbo,
        messages: vec![
            Message {
                role: Role::System,
                content: vec![
                    role_description.to_string(),
                    "Description of the game setting: ".to_string(),
                    description.to_string(),
                    "History that story is base on: ".to_string(),
                    history.to_string(),
                    system.to_string(),
                    ].join(" "),
            },
            Message {
                role: Role::User,
                content: request,
            },
            // Message {
            //     role: Role::Assistant,
            //     content:"".to_string(),
            // }
        ],
        ..Default::default()
    };
    let response = client.chat(chat_input).await?;
    Ok(response)
}



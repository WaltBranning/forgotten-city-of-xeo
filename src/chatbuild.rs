use chat_gpt_lib_rs::{
    ChatGPTClient,
    ChatInput, 
    Message, 
    Model, 
    Role, 
    ChatResponse
};

// use serde::{Deserialize, Serialize};
// use serde_json::{Result as JsonResult, Value};

use crate::game::*;
// use tokio;
// use color_eyre::eyre::Ok;


const API_KEY: &str = "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";
const BASE_URL: &str = "https://api.openai.com";


pub async fn chat_interface(request: &str) -> ChatResponse {
    let token = AppData::chat_config().token;
    let client:ChatGPTClient = ChatGPTClient::new(&token, BASE_URL);
    let role_description = Settings::world_data().speaker_role;
    // let location = Settings::location_data("gardens".to_string());
    // println!("{:?}",location);
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



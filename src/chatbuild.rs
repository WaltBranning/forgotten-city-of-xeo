use chat_gpt_lib_rs::{ChatGPTClient, ChatInput, Message, Model, Role, ChatResponse};
use std::io::Error;
// use tokio;
// use color_eyre::eyre::Ok;


const API_KEY: &str = "";
const BASE_URL: &str = "https://api.openai.com";


pub async fn chat_interface(request: &str) -> ChatResponse {
    let client:ChatGPTClient = ChatGPTClient::new(API_KEY, BASE_URL);

    let chat_input = ChatInput {
        model: Model::Gpt3_5Turbo,
        messages: vec![
            Message {
                role: Role::System,
                content: "You are dungeon master narrating the text adventure The Forgotten City Of Xeo. You provide a robust and interesting journey with a feel of Indiana Jones, National Treasure, and especially take inspiration from the style of James Rollins sigma force novels with a writing style like Stephen King".to_string(),
            },
            Message {
                role: Role::User,
                content: request.to_string(),
            }
        ],
        ..Default::default()
    };
    let response = client.chat(chat_input).await.unwrap();
    
    // println!("{:?}", response.choices[0].message.content);
    response
}
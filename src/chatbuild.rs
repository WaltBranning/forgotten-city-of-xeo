use chat_gpt_lib_rs::{ChatGPTClient, ChatInput, Message, Model, Role};
use std::io
// use tokio;
// use color_eyre::eyre::Ok;


const API_KEY: &str = "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";
const BASE_URL: &str = "https://api.openai.com";
const CHAT_CLIENT = chatGPTClient::new(API_KEY, BASE_URL);

pub fn chat_interface(request: &str) -> Result<(),io::Error> {

    Ok(())
}
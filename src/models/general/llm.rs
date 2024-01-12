use serde::{Deserialize, Serialize};

// Represents a message that will be sent to ChatGPT
#[derive(Debug, Serialize, Clone)]
pub struct Message {
    // The role of the user sending the message
    pub role: String,
    // The content of the message
    pub content: String,
}

// Represents the data and options passed into ChatGPT
#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    // The ChatGPT model to use
    pub model: String,
    // The messages to pass into the model
    pub messages: Vec<Message>,
    // The temperature setting used to control the ChatGPT response
    pub temperature: f32,
}

// Represents the content of the API response from ChatGPT
#[derive(Debug, Deserialize)]
pub struct APIMessage {
    // The content of the response
    pub content: String,
}

// Represents the choices of the API response from ChatGPT
#[derive(Debug, Deserialize)]
pub struct APIChoice {
    // The message from the response
    pub message: APIMessage,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
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

use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

const CHAT_GPT_MODEL: &str = "gpt-4";

// Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API keys
    let api_key: String =
        env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in environment variables");
    let api_org: String =
        env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in environment variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new();

    // Create header for the API key
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create header for the ORG key
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client: Client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: CHAT_GPT_MODEL.to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting and debugging
    //     let res_raw = client
    //         .post(url)
    //         .json(&chat_completion)
    //         .send()
    //         .await
    //         .unwrap();
    //     dbg!(res_raw.text().await.unwrap());

    Ok("Place_holder".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        call_gpt(messages).await;
    }
}

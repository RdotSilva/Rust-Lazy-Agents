use serde::de::DeserializeOwned;

use reqwest::Client;

use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use std::env;

use std::fs;

/// Extend AI function to encourage specific output
/// This will help us get a specific output that we are expecting
/// This will run the AI function, get the string out of the function and extend the function string
/// # Arguments
///
/// * `ai_func` - The the function we want to extend
//
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str: &str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}
  INSTRUCTION: You are a function printer. You ONLY print the results of functions.
  Nothing else. No commentary. Here is the input to the function: {}.
  Print out what the function will return.",
        ai_function_str, func_input
    );

    // Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

/// Performs call to LLM GPT
/// # Arguments
///
/// * `msg_context` - The message we are going to send as a function input
/// * `agent_position` - The type of agent making the request
/// * `agent_operation` - The operation the agent is actually doing
/// * `function_pass` - The function that we are passing
///
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Return Success or try again
    match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    }
}

/// Performs call to LLM GPT - Decoded version
/// We will get the string back from LLM and decode the string and create a struct
/// # Arguments
///
/// * `msg_context` - The message we are going to send as a function input
/// * `agent_position` - The type of agent making the request
/// * `agent_operation` - The operation the agent is actually doing
/// * `function_pass` - The function that we are passing
///
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");

    return decoded_response;
}

/// Check whether request URL is valid
/// # Arguments
///
/// * `client` - The client making the request
/// * `url` - The URL of the request
///
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

/// Read the code template from the web server project
/// This string will be fed into the LLM
pub fn read_code_template_contents() -> String {
    let code_template_path: String = env::var("CODE_TEMPLATE_PATH")
        .expect("CODE_TEMPLATE_PATH not found in environment variables");

    let path: String = String::from(code_template_path);
    fs::read_to_string(path).expect("Failed to read code template")
}

/// Read the final version of the executable code
pub fn read_exec_main_contents() -> String {
    let exec_main_path: String =
        env::var("EXEC_MAIN_PATH").expect("EXEC_MAIN_PATH not found in environment variables");

    let path: String = String::from(exec_main_path);
    fs::read_to_string(path).expect("Failed to read code template")
}

/// Save the backend code to file
pub fn save_backend_code(contents: &String) {
    let exec_main_path: String =
        env::var("EXEC_MAIN_PATH").expect("EXEC_MAIN_PATH not found in environment variables");

    let path: String = String::from(exec_main_path);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

/// Save JSON API Endpoint Schema to a file
pub fn save_api_endpoints(api_endpoints: &String) {
    let api_schema_path: String =
        env::var("API_SCHEMA_PATH").expect("API_SCHEMA_PATH not found in environment variables");

    let path: String = String::from(api_schema_path);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String =
            "Build me a web server for making stock price api requests.".to_string();

        let res: String = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}

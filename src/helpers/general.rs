use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;

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
    // TODO: Add implementation
    return "TODO".to_string();
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
}

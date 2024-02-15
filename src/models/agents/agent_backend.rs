use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    check_status_code, read_code_template_contents, save_api_endpoints, save_backend_code, read_exec_main_contents
};

use crate::helpers::command_line::PrintCommand
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, RouteObject, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;


/// Represents a backend developer agent
#[derive(Debug)]
pub struct AgentBackendDeveloper {
    /// The attributes belonging to the agent
    attributes: BasicAgent,
    /// Bug errors that are stored if they are encountered by the agent
    bug_errors: Option<String>,
    /// Total number of bug errors stored
    bug_count: u8,
}

/// Implementation for a Backend Developer Agent
impl AgentBackendDeveloper {
    /// Create a new Backend Developer Agent
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    /// Call the initial backend code provided as instructions
    /// Think of this as code written as a Junior Developer
    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

        // Concatenate instructions
        let msg_context: String = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_str, factsheet.project_description
        );

        // Generate a response from the AI model
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        // Save the backend code to the factsheet
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    /// Call the improved backend code provided from the factsheet
    /// Think of this as code improved by a Senior Developer
    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    /// Fix any backend code that has been flagged to have bugs
    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
      THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    /// Extract the REST API endpoints
    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code: String = read_exec_main_contents();

        // Structure message context
        let msg_context: String = format!("CODE_INPUT: {}", backend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        ai_response
    }

}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }


    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {

        while self.attributes.state != AgentState::Finished {
            
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                }
                AgentState::UnitTesting => {
                    // TODO: Update logic keep this as placeholder for now
                    self.attributes.state = AgentState::Finished;
                }
                AgentState::Finished => {}
                _ => {}
               }
            }
            Ok(())
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
   // TODO: Add test to check backend_developer
}
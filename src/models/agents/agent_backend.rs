use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    check_status_code, read_code_template_contents, save_api_endpoints, save_backend_code,
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
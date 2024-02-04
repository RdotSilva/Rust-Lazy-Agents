use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::general::llm::Message;

/// Represents a managing agent
#[derive(Debug)]
pub struct ManagingAgent {
    /// Attributes belonging to the agent
    _attributes: BasicAgent,
    /// Fact sheet for the agent
    factsheet: FactSheet,
    /// A group of agents that have access to special functions
    agents: Vec<Box<dyn SpecialFunctions>>,
}

// TODO: Add ManagingAgent implementation
impl ManagingAgent {}

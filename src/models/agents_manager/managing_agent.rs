use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

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

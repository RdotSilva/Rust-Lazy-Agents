use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::FactSheet;

/// Represents a managing agent
#[derive(Debug)]
pub struct ManagingAgent {
    /// Attributes belonging to the agent
    _attributes: BasicAgent,
    /// Fact sheet for the agent
    factsheet: FactSheet,
    agents: Vec, // TODO: Add type
}

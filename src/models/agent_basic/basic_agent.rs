use crate::models::general::llm::Message;

/// Enum to describe the different states of an agent
#[derive(Debug, PartialEq)]
pub enum AgentState {
    /// Discovery state
    Discovery,
    /// Working state
    Working,
    /// Unit Testing state
    UnitTesting,
    /// Finished state
    Finished,
}

/// Represents a basic agent
#[derive(Debug)]
pub struct BasicAgent {
    /// The main objective of the agent
    pub objective: String,
    /// The position of the agent
    pub position: String,
    /// The current state of an agent
    pub state: AgentState,
    /// The memory of an agent
    /// This is where we will store any conversation history
    pub memory: Vec<Message>,
}

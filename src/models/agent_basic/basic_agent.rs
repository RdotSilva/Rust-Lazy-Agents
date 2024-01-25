use crate::models::agent_basic::basic_traits::BasicTraits;
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

/// Represents an implementation for a basic agent using basic traits
impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery, // An agent should always start in discovery mode
            memory: Vec::from([]),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}

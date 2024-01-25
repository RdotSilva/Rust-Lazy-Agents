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

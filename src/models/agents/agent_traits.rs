use crate::models::agent_basic::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Represents a set of rules that the agents should follow
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
    /// Description of the project
    pub project_description: String,
    /// Scope of the project
    pub project_scope: Option<ProjectScope>,
    /// External URLS related to the project
    pub external_urls: Option<Vec<String>>,
    /// Backend code related to the project
    pub backend_code: Option<String>,
    /// API endpoint schema for the project
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

use crate::models::agent_basic::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Represents the schema for the API endpoints
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    /// Flag to determine if route is dynamic
    pub is_route_dynamic: String,
    /// The method of the route
    pub method: String,
    /// The request body of the route
    pub request_body: serde_json::Value,
    /// The response of the route
    pub response: serde_json::Value,
    /// The route path
    pub route: String,
}

/// Represents the scope of the project
/// This will be unknown when we first create a FactSheet
/// This will be generated only after the agent does the initial work
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
    /// Flag to determine if CRUD will be used
    pub is_crud_required: bool,
    /// Flag to determine if login/logout will be used
    pub is_user_login_and_logout: bool,
    /// Flag to determine if external URLS will be used
    pub is_external_urls_required: bool,
}

/// Represents a set of rules that the agents should follow
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
    /// Description of the project
    pub project_description: String,
    /// Scope of the project (this will start as None when we start a project)
    pub project_scope: Option<ProjectScope>,
    /// External URLS related to the project
    pub external_urls: Option<Vec<String>>,
    /// Backend code related to the project
    pub backend_code: Option<String>,
    /// API endpoint schema for the project
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

/// Declarative macro to turn a function into a string
#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::ManagingAgent;

fn main() {
    let user_request: String = get_user_response("What website would you like to build today?");

    let mut manage_agent: ManagingAgent = ManagingAgent::new(user_request)
        .await
        .expect("Error creating agent");

    manage_agent.execute_project().await;

    dbg!(manage_agent);
}

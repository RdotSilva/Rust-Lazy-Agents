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
fn main() {
    let user_request: String = get_user_response("What web server would you like to build today?");

    dbg!(user_request);
}

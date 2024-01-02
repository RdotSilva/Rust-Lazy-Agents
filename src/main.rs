mod ai_functions;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
fn main() {
    let user_request: String = get_user_response("What web server would you like to build today?");

    dbg!(user_request)
}

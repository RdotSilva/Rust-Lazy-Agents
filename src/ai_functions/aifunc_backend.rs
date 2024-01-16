use ai_functions::ai_function;

#[ai_function]
pub fn print_backend_webserver_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
    /// IMPORTANT: The backend code is ONLY an example. If the Project Description requires it, make as many changes as you like.
    /// IMPORTANT: You do not need to follow the backend code exactly. Write functions that make sense for the users request if required.
    /// FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in the PROJECT_DESCRIPTION
    /// IMPORTANT: The following libraries are already installed
    ///   reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    /// No other external libraries should be used. Write functions that fit with the description from the PROJECT_DESCRIPTION
    /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

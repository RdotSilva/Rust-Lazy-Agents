/// Extend AI function to encourage specific output
/// This will help us get a specific output that we are expecting
/// This will run the AI function, get the string out of the function and extend the function string
/// # Arguments
///
/// * `ai_func` - The the function we want to extend
//
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) {
    let ai_function_str: &str = ai_func(func_input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg: &str = convert_user_input_to_goal("Dummy Variable");
        dbg!(&extended_msg);
    }
}

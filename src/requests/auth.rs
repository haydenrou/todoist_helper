use std::env;

pub fn todoist_token() -> String {
    match env::var("TODOIST_TOKEN") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    }
}

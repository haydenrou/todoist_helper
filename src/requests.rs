use crate::PostTodo;
use crate::input::get_user_input;
use crate::Todo;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

mod auth;

// static PROJECTS_URL: &str = "https://api.todoist.com/rest/v1/projects";
static TASKS_URL: &str = "https://api.todoist.com/rest/v1/tasks";

pub async fn add_todo(initial_values: PostTodo) -> Result<Todo, reqwest::Error> {
    let request_url = format!("{}", TASKS_URL);
    let auth_key: &str = &("Bearer ".to_owned() + &auth::todoist_token().to_owned());
    let data: PostTodo = PostTodo {
        content: if initial_values.content.is_empty() { get_user_input("content".to_string()) } else { initial_values.content },
        description: initial_values.description
    };
    let data_as_json = serde_json::to_string(&data).expect("No data");

    let client = reqwest::Client::new();
    let response = client
        .post(&request_url)
        .header(AUTHORIZATION, auth_key)
        .header(CONTENT_TYPE, "application/json")
        .body(data_as_json)
        .send()
        .await?
        .text()
        .await?;

    let result = serde_json::from_str::<Todo>(&response).expect("No Todo found");

    Ok(result)
}


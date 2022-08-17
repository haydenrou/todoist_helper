use crate::PostTodo;
use crate::input::get_user_input;
use crate::Todo;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;

mod auth;


pub enum RequestResponse {
    Single(Todo),
    Multiple(Vec<Todo>)
}

// static PROJECTS_URL: &str = "https://api.todoist.com/rest/v1/projects";
static TASKS_URL: &str = "https://api.todoist.com/rest/v1/tasks";

pub async fn add_todo(initial_values: PostTodo) -> Result<RequestResponse, reqwest::Error> {
    let auth_key: &str = &("Bearer ".to_owned() + &auth::todoist_token().to_owned());

    let request_url = format!("{}", TASKS_URL);
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

    Ok(RequestResponse::Single(result))
}

// today or overdue
pub async fn show_tasks(date_string: String) -> Result<RequestResponse, reqwest::Error> {
    let auth_key: &str = &("Bearer ".to_owned() + &auth::todoist_token().to_owned());

    let request_url = format!("{}", TASKS_URL);

    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("filter", date_string);

    println!("{:?}", params);
    let response = client
        .get(request_url)
        .query(&params)
        .header(AUTHORIZATION, auth_key)
        .send()
        .await?
        .text()
        .await?;

    let result_set = serde_json::from_str::<Vec<Todo>>(&response).expect("No Todos found");
    for todo in &result_set {
        println!("{}", todo.content)
    };

    Ok(RequestResponse::Multiple(result_set))
}

use crate::PostTodo;
use crate::input::get_user_input;
use crate::Todo;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;

mod auth;

pub enum RequestResponse {
    Single(Todo),
    Multiple(Vec<Todo>),
    Empty(()),
}

// static PROJECTS_URL: &str = "https://api.todoist.com/rest/v1/projects";
static TASKS_URL: &str = "https://api.todoist.com/rest/v1/tasks/";

pub fn add_todo(initial_values: PostTodo) -> Result<RequestResponse, reqwest::Error> {
    let auth_key: &str = &("Bearer ".to_owned() + &auth::todoist_token().to_owned());

    let request_url = format!("{}", TASKS_URL);
    let data: PostTodo = PostTodo {
        content: if initial_values.content.is_empty() { get_user_input("content".to_string()) } else { initial_values.content },
        description: initial_values.description,
        due_string: initial_values.due_string
    };
    let data_as_json = serde_json::to_string(&data).expect("No data");

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&request_url)
        .header(AUTHORIZATION, auth_key)
        .header(CONTENT_TYPE, "application/json")
        .body(data_as_json)
        .send()?;

    let result = response.json::<Todo>();

    let todo = match result {
        Ok(todo) => {
            println!("TODO created with the following details");
            println!("ID: {}", todo.id);
            println!("Content: {}", todo.content);
            todo
        },
        Err(e) => {
            println!("{}", e);
            unreachable!()
        }
    };

    Ok(RequestResponse::Single(todo))
}

pub fn show_todos(filter_by: String, attributes_to_collect: Vec<String>) -> Result<RequestResponse, reqwest::Error> {
    let auth_key: &str = &("Bearer ".to_owned() + &auth::todoist_token().to_owned());

    let request_url = format!("{}", TASKS_URL);

    let client = reqwest::blocking::Client::new();

    let mut params = HashMap::new();
    params.insert("filter", filter_by);

    let response = client
        .get(request_url)
        .query(&params)
        .header(AUTHORIZATION, auth_key)
        .send()?;

    let result_set = response.json::<Vec<Todo>>();

    let results = match result_set {
        Ok(result) => {
            for todo in &result {
                if attributes_to_collect[0] == "" {
                    println!("{}", todo.content)
                } else {
                    for attribute in &attributes_to_collect {
                        if attribute == "id" {
                            println!("{:?}", todo.id)
                        } else if attribute == "content" {
                            println!("{:?}", todo.content)
                        }
                    }
                }
            }

            result
        },
        Err(e) => {
            println!("{}", e);
            unreachable!()
        }
    };

    Ok(RequestResponse::Multiple(results))
}

pub fn complete_todo(id: Option<u64>) -> Result<RequestResponse, reqwest::Error> {
    let auth_key: &str = &("Bearer ".to_owned() + &auth::todoist_token().to_owned());
    let parsed_id = match id {
        Some(value) => { value.to_string() },
        None => "".to_string()
    };

    if parsed_id.is_empty() {
        println!("not a valid ID");
        unreachable!();
    }

    let request_url = format!("{}", TASKS_URL.to_owned() + &(parsed_id) + &("/close"));

    let client = reqwest::blocking::Client::new();

    match client.post(request_url).header(AUTHORIZATION, auth_key).send() {
        Ok(result) => {
            println!("Your Todo has been successfully completed.");
            result
        },
        Err(e) => {
            println!("{}", e);
            unreachable!()
        }
    };


    Ok(RequestResponse::Empty(()))
}

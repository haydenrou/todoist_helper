use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::stdin;
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct Due {
    date: String,
    recurring: bool,
    lang: String,
    string: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i64,
    assigner: i64,
    project_id: u32,
    section_id: u32,
    content: String,
    completed: bool,
    description: String,
    due: Option<Due>,
    order: i64,
    priority: u32,
    comment_count: u32,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct PostTodo {
    content: String,
    description: String
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Project {
//     id: u32,
//     name: String,
//     comment_count: u32,
//     color: u32,
//     shared: bool,
//     sync_id: u32,
//     favorite: bool,
//     url: String,
// }

// static PROJECTS_URL: &str = "https://api.todoist.com/rest/v1/projects";
static TASKS_URL: &str = "https://api.todoist.com/rest/v1/tasks";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    action: String,
    target: String,

    #[clap(short = 'c', value_name = "CONTENT")]
    content: Option<String>,

    #[clap(short = 'd', value_name = "DESCRIPTION")]
    description: Option<String>,

}

fn main() {
    let args = Cli::parse();
    let rt  = Runtime::new().unwrap();

    rt.block_on(async {
        let arg_action = match [&*args.action, &*args.target] {
            ["add", "todo"] => add_todo(PostTodo {
                content: match args.content {
                    Some(value) => { value.trim().to_string() },
                    None => "".to_string()
                },
                description: match args.description {
                    Some(value) => { value.trim().to_string() },
                    None => "".to_string()
                }
            }).await,
            _ => unreachable!()
        };
        if arg_action.is_ok() {
            println!("Your task has been successfully created!")
        } else if arg_action.is_err() {
            println!("Your task has had an error.")
        };
    });
}

async fn add_todo(initial_values: PostTodo) -> Result<Todo, reqwest::Error> {
    let request_url = format!("{}", TASKS_URL);
    let auth_key: &str = &("Bearer ".to_owned() + &todoist_token().to_owned());
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

fn get_user_input(field: String) -> String {
    println!("{}", "What is the ".to_owned() + &field.to_owned());
    let mut text = String::new();
    stdin().read_line(&mut text)
        .ok()
        .expect(&("Not a valid ".to_owned() + &field.to_owned()));

    text
}

fn todoist_token() -> String {
    match env::var("TODOIST_TOKEN") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    }
}

mod requests;
mod input;

use crate::requests::{show_todos, add_todo, complete_todo};
use requests::RequestResponse;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct Due {
    date: String,
    recurring: bool,
    string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
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
pub struct PostTodo {
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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    action: String,
    target: String,
    item: Option<u64>,

    #[clap(short = 'c', value_name = "CONTENT")]
    content: Option<String>,

    #[clap(short = 'd', value_name = "DESCRIPTION")]
    description: Option<String>,

    #[clap(short = 'f', value_name = "FILTERS")]
    filters: Option<String>,

    #[clap(short = 'a', value_name = "ATTRIBUTES")]
    attributes: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let arg_action: Result<RequestResponse, reqwest::Error> = match [&*args.action, &*args.target] {
        ["add", "todo"] => add_todo(PostTodo {
            content: match args.content {
                Some(value) => { value.trim().to_string() },
                None => "".to_string()
            },
            description: match args.description {
                Some(value) => { value.trim().to_string() },
                None => "".to_string()
            }
        }),
        ["show", "todos"] => show_todos(
            match args.filters {
                Some(value) => { value.trim().to_string() },
                None => "".to_string()
            },
            match args.attributes {
                Some(ref values) => { values.split(",").map(|s|s.to_string()).collect() },
                None => ["".to_string()].to_vec()
            }
        ),
        ["complete", "todo"] => complete_todo(args.item),
        _ => unreachable!()
    };

    if arg_action.is_err() {
        println!("Your task has had an error.")
    };
}


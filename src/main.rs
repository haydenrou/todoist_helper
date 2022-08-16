mod requests;
mod input;

use crate::requests::add_todo;
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct Due {
    date: String,
    recurring: bool,
    lang: String,
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


use reqwest::header::AUTHORIZATION;
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: u32,
    name: String,
    comment_count: u32,
    color: u32,
    shared: bool,
    sync_id: u32,
    favorite: bool,
    url: String,
}

fn main() -> Result<(), reqwest::Error> {
    let rt  = Runtime::new().unwrap();
    rt.block_on(async {
        let response = get_project().await;
        println!("{:#?}", response.expect("Results"))
    });
    Ok(())
}

async fn get_project() -> Result<Vec<Project>, reqwest::Error> {
    let request_url = format!("https://api.todoist.com/rest/v1/projects");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let auth_key: &str = &("Bearer ".to_owned() + &todoist_token().to_owned());
    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, auth_key)
        .send()
        .await?
        .text()
        .await?;

    let result = serde_json::from_str::<Vec<Project>>(&response).expect("No projects");

    Ok(result)
}

fn todoist_token() -> String {
    match env::var("TODOIST_TOKEN") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    }
}

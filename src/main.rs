use reqwest::Error;
use reqwest::header::AUTHORIZATION;
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: u32,
    name: String,
    comment_count: u32,
    // order: u32,
    color: u32,
    shared: bool,
    sync_id: u32,
    favorite: bool,
    // inbox_project: bool,
    url: String,
    // login: String,
}

fn main() -> Result<(), Error> {
    let rt  = Runtime::new().unwrap();
    rt.block_on(async {
        let response = get_project().await;
        println!("{:#?}", response)
    });
    Ok(())
}

async fn get_project() -> Result<(), reqwest::Error> {
    let request_url = format!("https://api.todoist.com/rest/v1/projects");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, "Bearer test")
        .send()
        .await?
        .text()
        .await?;
    // println!("Response Body: {}", response);
    let result = serde_json::from_str::<Vec<Project>>(&response);
        // .await
        // .expect("something")
        // .json::<GETAPIResponse>().await?;

    print!("{:?}",result);
    Ok(())
}

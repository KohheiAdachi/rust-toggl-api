extern crate serde_json;
extern crate tokio;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Method;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct TimeEntry {
    at: String,
    billable: bool,
    description: String,
    duration: i64,
    duronly: bool,
    id: i64,
    pid: i64,
    project_id: Option<i64>,
    server_deleted_at: Option<String>,
    start: String,
    stop: Option<String>,
    tag_ids: Vec<i64>,
    tags: Vec<String>,
    task_id: Option<i64>,
    uid: i64,
    user_id: i64,
    wid: i64,
    workspace_id: i64,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_token = match std::env::var("API_TOKEN") {
        Ok(val) => val,
        Err(_e) => "dev".to_string(),
    };

    let res = client
        .request(
            Method::GET,
            "https://api.track.toggl.com/api/v9/me/time_entries".to_string(),
        )
        // HACK: passwordがOption<P>なので、Some("api_token")を渡す
        .basic_auth(api_token, Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    // let body = res.text().await?;
    let body = res.json::<Vec<TimeEntry>>().await?;
    println!("{:?}", body);

    Ok(())
}

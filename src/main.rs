extern crate serde_json;
extern crate tokio;
use chrono::{DateTime, FixedOffset, Local, Timelike};
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

fn get_start_date() -> String {
    let dt = Local::now();
    let start_time = dt
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .to_rfc3339()
        .to_string();
    return start_time;
}

fn get_end_date() -> String {
    let dt = Local::now();
    let start_time = dt
        .with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
        .to_rfc3339()
        .to_string();
    return start_time;
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_token = match std::env::var("API_TOKEN") {
        Ok(val) => val,
        Err(_e) => "dev".to_string(),
    };

    let start_time = get_start_date();
    let end_time = get_end_date();

    let res = client
        .request(
            Method::GET,
            "https://api.track.toggl.com/api/v9/me/time_entries".to_string(),
        )
        .query(&[("start_date", start_time), ("end_date", end_time)])
        // HACK: passwordがOption<P>なので、Some("api_token")を渡す
        .basic_auth(api_token, Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    // let body = res.text().await?;
    let body = res.json::<Vec<TimeEntry>>().await?;
    println!("{:?}", body);
    let reverse = body.iter().rev();
    println!("|作業内容|時間|開始|終了|");
    println!("|------|-|-----|------|");
    for entry in reverse {
        if entry.stop == None {
            continue;
        }
        let start = DateTime::parse_from_rfc3339(&entry.start).unwrap();

        let start_jst = start.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());

        let stop = DateTime::parse_from_rfc3339(&entry.stop.clone().unwrap()).unwrap();

        let stop_jst = stop.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());

        println!(
            "|{}|{:0>2}:{:0>2}|{:0>2}:{:0>2}|{:0>2}:{:0>2}|",
            entry.description,
            entry.duration / 3600,
            (entry.duration % 3600) / 60,
            start_jst.hour(),
            start_jst.minute(),
            stop_jst.hour(),
            stop_jst.minute()
        );
    }

    Ok(())
}

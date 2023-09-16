extern crate serde_json;
extern crate tokio;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Method;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_token = match std::env::var("API_TOKEN") {
        Ok(val) => val,
        Err(_e) => "dev".to_string(),
    };

    let json = client
        .request(
            Method::GET,
            "https://api.track.toggl.com/api/v9/me/time_entries".to_string(),
        )
        // HACK: passwordがOption<P>なので、Some("api_token")を渡す
        .basic_auth(api_token, Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    println!("{:#?}", json);

    Ok(())
}

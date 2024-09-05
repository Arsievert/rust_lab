use reqwest::Error;
use serde::Deserialize;
use tokio::time::{self, Duration};

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
}

async fn poll_token_service() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://token_service:3000/token")
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    println!("Received token: {}", response.token);
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        match poll_token_service().await {
            Ok(_) => (),
            Err(e) => eprintln!("Error fetching token: {}", e),
        }
    }
}

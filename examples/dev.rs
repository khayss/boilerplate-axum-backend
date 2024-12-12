use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Create an HTTP client
    let client = Client::new();

    // Base URL of your server
    let base_url = "http://127.0.0.1:4000";

    // 1. Health Check (GET /health)
    let response = client.get(format!("{}/health", base_url)).send().await?;

    let text = response.text().await?;
    println!("Health Check Response: {text}");

    Ok(())
}

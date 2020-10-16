mod github;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    github::login().await;
    Ok(())
}
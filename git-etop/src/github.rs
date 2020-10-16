extern crate reqwest;
use self::reqwest::Client;
use self::reqwest::header;

const GITHUB_API_URL: &'static str =  "https://api.github.com";

struct GithubAuth {
    username: String,
    name: String,
    default_repo: String,
    token: String,
}

pub async fn login() -> Result<(), Box<dyn std::error::Error>> {
    println!("Go to https://github.com/settings/tokens and generate token with permissions: [read:user, write:discussion]");
    println!("Enter token: ");

    let mut githubToken = String::new();

    std::io::stdin()
        .read_line(&mut githubToken)
        .expect("Failed to read line");

    let request_url = "https://api.github.com/user";
    let auth = format!("token {}", githubToken);

    let client = Client::new()
        .get(request_url)
        .header(header::AUTHORIZATION, "token a8e92a149b4f0f931fd1b267766a52403597a4fe")
        .send()
        .await;
    println!("RESPONSE {} {} \n{:?}",request_url, auth, client);

    Ok(())
}
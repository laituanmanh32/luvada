extern crate hyper;

use hyper::{Body, Method, Request, Uri};

const GITHUB_API_URL: &'static str =  "https://api.github.com";

struct GithubAuth {
    username: String,
    name: String,
    default_repo: String,
    token: String,
}

pub async fn login() -> Result<(), Box<dyn std::error::Error>> {
    println!("Go  to https://github.com/settings/tokens and generate token with permissions: [read:user, write:discussion]");
    println!("Enter token: ");

    // let mut githubToken = String::new();
    //
    // std::io::stdin()
    //     .read_line(&mut githubToken)
    //     .expect("Failed to read line");
    //
    // let request_url = "https://api.github.com/user";
    // let auth = format!("token {}", githubToken);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    // println!("RESPONSE {} {} \n{:?}",request_url, auth, client);

    println!("CHECK {:?}", req);

    Ok(())
}
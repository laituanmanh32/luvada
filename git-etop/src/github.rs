struct GithubAuth {
    username: String,
    name: String,
    default_repo: String,
    token: String,
}

pub async fn login() {
    println!("Go to https://github.com/settings/tokens and generate token with permissions: [read:user, write:discussion]");
    println!("Enter token: ");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
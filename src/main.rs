fn get_github_personal_access_token() -> String {
    // get token from environment variable
    let token = std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap();
    token
}

fn main() {
    let token = get_github_personal_access_token();
    println!("{}", token);
}

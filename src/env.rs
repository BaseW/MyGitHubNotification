pub fn get_github_personal_access_token() -> String {
    // get token from environment variable
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap()
}

pub fn get_slack_webhook_url_from_env() -> String {
    // get token from environment variable
    std::env::var("SLACK_WEBHOOK_URL").unwrap()
}

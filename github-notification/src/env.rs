pub fn get_github_personal_access_token() -> String {
    // get token from environment variable
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap()
}

pub fn get_slack_webhook_url_from_env() -> String {
    // get token from environment variable
    std::env::var("SLACK_WEBHOOK_URL").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_github_personal_access_token_1() {
        std::env::set_var("GITHUB_PERSONAL_ACCESS_TOKEN", "test_token");
        let token = get_github_personal_access_token();
        assert_eq!(token, "test_token");
    }

    #[test]
    #[should_panic]
    fn test_get_github_personal_access_token_2() {
        // reset environment variable
        std::env::remove_var("GITHUB_PERSONAL_ACCESS_TOKEN");
        get_github_personal_access_token();
    }

    #[test]
    fn test_get_slack_webhook_url_from_env_1() {
        std::env::set_var("SLACK_WEBHOOK_URL", "test_url");
        let url = get_slack_webhook_url_from_env();
        assert_eq!(url, "test_url");
    }

    #[test]
    #[should_panic]
    fn test_get_slack_webhook_url_from_env_2() {
        // reset environment variable
        std::env::remove_var("SLACK_WEBHOOK_URL");
        get_slack_webhook_url_from_env();
    }
}

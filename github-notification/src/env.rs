pub const GITHUB_PERSONAL_ACCESS_TOKEN_KEY: &str = "GITHUB_PERSONAL_ACCESS_TOKEN";
pub const SLACK_WEBHOOK_URL_KEY: &str = "SLACK_WEBHOOK_URL";
pub const SLACK_SLASH_COMMAND_TOKEN_KEY: &str = "SLACK_SLASH_COMMAND_TOKEN";

pub fn get_github_personal_access_token() -> String {
    // get token from environment variable
    std::env::var(GITHUB_PERSONAL_ACCESS_TOKEN_KEY).unwrap()
}

pub fn get_slack_webhook_url_from_env() -> String {
    // get token from environment variable
    std::env::var(SLACK_WEBHOOK_URL_KEY).unwrap()
}

pub fn get_slack_slash_command_token() -> String {
    // get token from environment variable
    std::env::var(SLACK_SLASH_COMMAND_TOKEN_KEY).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_github_personal_access_token_1() {
        let test_token = "test_token";
        std::env::set_var(GITHUB_PERSONAL_ACCESS_TOKEN_KEY, test_token);
        let token = get_github_personal_access_token();
        assert_eq!(token, test_token);
    }

    #[test]
    #[should_panic]
    fn test_get_github_personal_access_token_2() {
        // reset environment variable
        std::env::remove_var(GITHUB_PERSONAL_ACCESS_TOKEN_KEY);
        get_github_personal_access_token();
    }

    #[test]
    fn test_get_slack_webhook_url_from_env_1() {
        let test_url = "test_url";
        std::env::set_var(SLACK_WEBHOOK_URL_KEY, test_url);
        let url = get_slack_webhook_url_from_env();
        assert_eq!(url, test_url);
    }

    #[test]
    #[should_panic]
    fn test_get_slack_webhook_url_from_env_2() {
        // reset environment variable
        std::env::remove_var(SLACK_WEBHOOK_URL_KEY);
        get_slack_webhook_url_from_env();
    }

    #[test]
    fn test_get_slack_slash_command_token_1() {
        let test_token = "test_token";
        std::env::set_var(SLACK_SLASH_COMMAND_TOKEN_KEY, test_token);
        let token = get_slack_slash_command_token();
        assert_eq!(token, test_token)
    }

    #[test]
    #[should_panic]
    fn test_get_slack_slash_command_token_2() {
        // reset environment variable
        std::env::remove_var(SLACK_SLASH_COMMAND_TOKEN_KEY);
        get_slack_slash_command_token();
    }
}

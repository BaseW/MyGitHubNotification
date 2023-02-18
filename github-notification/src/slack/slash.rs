use crate::env::get_slack_slash_command_token;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlashCommandPayload {
    pub token: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
}

#[derive(Debug)]
pub struct SlackCommandRequest {
    pub command: String,
    pub text: String,
}

const AVAILABLE_COMMANDS: [&str; 1] = ["/mygithub"];
const AVAILABLE_TEXT: [&str; 3] = ["help", "health-check", "create-notification"];

pub fn validate_slash_command_payload(
    payload: &SlashCommandPayload,
) -> Result<SlackCommandRequest, String> {
    if payload.token != get_slack_slash_command_token() {
        return Err("Invalid token".to_string());
    }

    if !AVAILABLE_COMMANDS.contains(&payload.command.as_str()) {
        return Err("Invalid command".to_string());
    }

    if !AVAILABLE_TEXT.contains(&payload.text.as_str()) {
        return Err("Invalid text".to_string());
    }

    Ok(SlackCommandRequest {
        command: payload.command.clone(),
        text: payload.text.clone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::{env::SLACK_SLASH_COMMAND_TOKEN_KEY, slack::slash::SlashCommandPayload};

    #[test]
    fn test_validate_slash_command_payload_valid_token() {
        let valid_token = "test_token";
        let valid_command = "/mygithub";
        let valid_text = "help";
        std::env::set_var(SLACK_SLASH_COMMAND_TOKEN_KEY, valid_token);
        let mock_payload = SlashCommandPayload {
            token: valid_token.to_string(),
            team_id: "test_team_id".to_string(),
            team_domain: "test_team_domain".to_string(),
            channel_id: "test_channel_id".to_string(),
            channel_name: "test_channel_name".to_string(),
            user_id: "test_user_id".to_string(),
            user_name: "test_user_name".to_string(),
            command: valid_command.to_string(),
            text: valid_text.to_string(),
            response_url: "test_response_url".to_string(),
        };
        let result = super::validate_slash_command_payload(&mock_payload);
        assert!(result.is_ok());
        let slack_command_request = result.unwrap();
        assert_eq!(slack_command_request.command, valid_command);
        assert_eq!(slack_command_request.text, valid_text);
    }

    #[test]
    fn test_validate_slash_command_payload_invalid_token() {
        let valid_token = "test_token";
        let invalid_token = "invalid_token";
        let valid_command = "/mygithub";
        let valid_text = "help";
        std::env::set_var(SLACK_SLASH_COMMAND_TOKEN_KEY, valid_token);
        let mock_payload = SlashCommandPayload {
            token: invalid_token.to_string(),
            team_id: "test_team_id".to_string(),
            team_domain: "test_team_domain".to_string(),
            channel_id: "test_channel_id".to_string(),
            channel_name: "test_channel_name".to_string(),
            user_id: "test_user_id".to_string(),
            user_name: "test_user_name".to_string(),
            command: valid_command.to_string(),
            text: valid_text.to_string(),
            response_url: "test_response_url".to_string(),
        };
        let result = super::validate_slash_command_payload(&mock_payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid token");
    }

    #[test]
    fn test_validate_slash_command_payload_invalid_command() {
        let valid_token = "test_token";
        let invalid_command = "/invalid_command";
        let valid_text = "help";
        std::env::set_var(SLACK_SLASH_COMMAND_TOKEN_KEY, valid_token);
        let mock_payload = SlashCommandPayload {
            token: valid_token.to_string(),
            team_id: "test_team_id".to_string(),
            team_domain: "test_team_domain".to_string(),
            channel_id: "test_channel_id".to_string(),
            channel_name: "test_channel_name".to_string(),
            user_id: "test_user_id".to_string(),
            user_name: "test_user_name".to_string(),
            command: invalid_command.to_string(),
            text: valid_text.to_string(),
            response_url: "test_response_url".to_string(),
        };
        let result = super::validate_slash_command_payload(&mock_payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid command");
    }

    #[test]
    fn test_validate_slash_command_payload_invalid_text() {
        let valid_token = "test_token";
        let valid_command = "/mygithub";
        let invalid_text = "invalid_text";
        std::env::set_var(SLACK_SLASH_COMMAND_TOKEN_KEY, valid_token);
        let mock_payload = SlashCommandPayload {
            token: valid_token.to_string(),
            team_id: "test_team_id".to_string(),
            team_domain: "test_team_domain".to_string(),
            channel_id: "test_channel_id".to_string(),
            channel_name: "test_channel_name".to_string(),
            user_id: "test_user_id".to_string(),
            user_name: "test_user_name".to_string(),
            command: valid_command.to_string(),
            text: invalid_text.to_string(),
            response_url: "test_response_url".to_string(),
        };
        let result = super::validate_slash_command_payload(&mock_payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid text");
    }
}

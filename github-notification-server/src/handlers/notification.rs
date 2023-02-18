use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use github_notification::{
    env::{get_github_personal_access_token, get_slack_webhook_url_from_env},
    github::{get_my_issues, sort_issues},
    slack::{
        notification::notify_by_slack,
        payload::create_payload_for_slack,
        slash::{validate_slash_command_payload, SlashCommandPayload},
    },
};

// receive request from slack
// respond with text
#[debug_handler]
pub async fn create_notification_handler(
    Json(input): Json<SlashCommandPayload>,
) -> impl IntoResponse {
    // check token, command, text
    match validate_slash_command_payload(&input) {
        Ok(req) => {
            println!("req: {:?}", req);
        }
        Err(e) => {
            return (StatusCode::BAD_REQUEST, e);
        }
    }

    let token = get_github_personal_access_token();
    let webhook_url = get_slack_webhook_url_from_env();

    let github_api_addr = "https://api.github.com".to_string();
    let my_issues = get_my_issues(github_api_addr, token).await;
    let sorted_issues = sort_issues(my_issues);
    let payload = create_payload_for_slack(sorted_issues);

    // notify by slack
    notify_by_slack(webhook_url, payload).await;
    (StatusCode::OK, "ok".to_string())
}

use github_notification::env::{
    get_github_personal_access_token, get_slack_webhook_url_from_env,
};
use github_notification::github::{get_my_issues, sort_issues};
use github_notification::sentry::initialize_sentry;
use github_notification::slack::{create_payload_for_slack, notify_by_slack};

#[tokio::main]
async fn main() {
    let _guard = initialize_sentry();

    let token = get_github_personal_access_token();
    let webhook_url = get_slack_webhook_url_from_env();

    let github_api_addr = "https://api.github.com".to_string();
    let my_issues = get_my_issues(github_api_addr, token).await;
    let sorted_issues = sort_issues(my_issues);
    let payload = create_payload_for_slack(sorted_issues);

    // notify by slack
    notify_by_slack(webhook_url, payload).await;
}

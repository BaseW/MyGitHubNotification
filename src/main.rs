use my_github_notification::github::get_my_issues;
use my_github_notification::slack::{create_payload_for_slack, notify_by_slack};

#[tokio::main]
async fn main() {
    let my_issues = get_my_issues().await;

    let payload = create_payload_for_slack(my_issues);

    // notify by slack
    notify_by_slack(payload).await;
}

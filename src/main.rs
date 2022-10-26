use my_github_notification::github::{get_my_issues, sort_issues};
use my_github_notification::slack::{create_payload_for_slack, notify_by_slack};

#[tokio::main]
async fn main() {
    let my_issues = get_my_issues().await;

    let sorted_issues = sort_issues(my_issues);

    let payload = create_payload_for_slack(sorted_issues);

    // notify by slack
    notify_by_slack(payload).await;
}

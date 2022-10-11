use crate::env::get_slack_webhook_url_from_env;
use crate::errors::GetIssueError;
use crate::models::Issue;
use serde_json::json;

pub async fn notify_by_slack(text: String) {
    let webhook_url = get_slack_webhook_url_from_env();
    let client = reqwest::Client::new();
    let res = client
        .post(webhook_url)
        .json(&json!({ "text": text }))
        .send()
        .await;
    match res {
        Ok(_res) => {
            println!("Notify by Slack OK");
        }
        Err(err) => {
            println!("Notify by Slack Error: {}", err);
        }
    }
}

pub fn create_payload_for_slack(issues: Result<Vec<Issue>, GetIssueError>) -> String {
    let mut payload = String::new();

    match issues {
        Ok(issues) => {
            payload.push_str("@channel\n優先度が高いタスク一覧\n");
            if issues.is_empty() {
                payload.push_str("なし");
            } else {
                for issue in issues {
                    let issue_url = issue.url;
                    let issue_title = issue.title;
                    payload.push_str(&format!("- <{}|{}>\n", issue_url, issue_title));
                }
            }
        }
        Err(e) => {
            payload = e.message;
        }
    }

    payload
}

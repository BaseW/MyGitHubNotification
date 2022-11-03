use crate::env::get_slack_webhook_url_from_env;
use crate::errors::GetIssueError;
use crate::models::{Issue, SortedIssues};
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

fn generate_text_with_header(header: &str, issues: &Vec<Issue>) -> String {
    let mut text = String::new();
    text.push_str(format!("{}\n", header).as_str());

    for issue in issues {
        let issue_url = &issue.html_url;
        let issue_title = &issue.title;
        let issue_labels = match &issue.labels {
            Some(labels) => {
                let mut label_names = String::new();
                for label in labels {
                    label_names.push_str(&label.name);
                    label_names.push(' ');
                }
                label_names
            }
            None => String::from(""),
        };
        let issue_repository = &issue.repository;
        text.push_str(&format!(
            "- <{}|{}>(<{}|{}>): {}\n",
            issue_url, issue_title, issue_repository.html_url, issue_repository.name, issue_labels
        ));
    }
    text
}

pub fn create_payload_for_slack(issues: Result<SortedIssues, GetIssueError>) -> String {
    let mut payload = String::new();

    match issues {
        Ok(issues) => {
            // add header
            payload.push_str("<!channel>\nタスク一覧\n");

            // add priority high issues
            let priority_high_issues = issues.priority_high_issues;
            if priority_high_issues.len() > 0 {
                let text = generate_text_with_header("優先度: 高", &priority_high_issues);
                payload.push_str(&text);
            }

            // add priority medium issues
            let priority_medium_issues = issues.priority_medium_issues;
            if priority_medium_issues.len() > 0 {
                let text = generate_text_with_header("優先度: 中", &priority_medium_issues);
                payload.push_str(&text);
            }

            // add priority low issues
            let priority_low_issues = issues.priority_low_issues;
            if priority_low_issues.len() > 0 {
                let text = generate_text_with_header("優先度: 低", &priority_low_issues);
                payload.push_str(&text);
            }

            // add priority none issues
            let priority_none_issues = issues.priority_none_issues;
            if priority_none_issues.len() > 0 {
                let text = generate_text_with_header("優先度: なし", &priority_none_issues);
                payload.push_str(&text);
            }
        }
        Err(e) => {
            payload = e.message;
        }
    }

    payload
}

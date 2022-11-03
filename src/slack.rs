use crate::env::get_slack_webhook_url_from_env;
use crate::errors::GetIssueError;
use crate::models::{Issue, SlackMessageBlock, SlackMessageBlockText, SortedIssues};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageBlocks {
    #[serde(rename = "type")]
    blocks_type: String,
    blocks: Vec<SlackMessageBlock>,
}

impl SlackMessageBlocks {
    pub fn new() -> Self {
        Self {
            blocks_type: "home".to_string(),
            blocks: vec![],
        }
    }

    fn add_block(&mut self, block: SlackMessageBlock) {
        self.blocks.push(block);
    }

    pub fn add_header_block(&mut self, text: String) {
        let block = SlackMessageBlock {
            block_type: "header".to_string(),
            text: Some(SlackMessageBlockText {
                text_type: "plain_text".to_string(),
                text,
            }),
        };
        self.add_block(block);
    }

    pub fn add_text_block(&mut self, text: String) {
        let block = SlackMessageBlock {
            block_type: "section".to_string(),
            text: Some(SlackMessageBlockText {
                text_type: "mrkdwn".to_string(),
                text,
            }),
        };
        self.add_block(block);
    }
}

pub async fn notify_by_slack(message_blocks: SlackMessageBlocks) {
    let webhook_url = get_slack_webhook_url_from_env();
    let client = reqwest::Client::new();
    let res = client.post(webhook_url).json(&message_blocks).send().await;
    match res {
        Ok(res) => {
            // if status code is 200, it means success
            if res.status() == 200 {
                println!("Notify by Slack OK");
            } else {
                println!("Notify by Slack Error: {}", res.status());
            }
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

pub fn create_payload_for_slack(issues: Result<SortedIssues, GetIssueError>) -> SlackMessageBlocks {
    let mut message_block = SlackMessageBlocks::new();

    match issues {
        Ok(issues) => {
            // add mention to the channel
            message_block.add_text_block("<!channel>\n".to_string());
            message_block.add_header_block("タスク一覧".to_string());

            // add priority high issues
            let priority_high_issues = issues.priority_high_issues;
            if priority_high_issues.len() > 0 {
                let text = generate_text_with_header("*優先度: 高*", &priority_high_issues);
                message_block.add_text_block(text);
            }

            // add priority medium issues
            let priority_medium_issues = issues.priority_medium_issues;
            if priority_medium_issues.len() > 0 {
                let text = generate_text_with_header("*優先度: 中*", &priority_medium_issues);
                message_block.add_text_block(text);
            }

            // add priority low issues
            let priority_low_issues = issues.priority_low_issues;
            if priority_low_issues.len() > 0 {
                let text = generate_text_with_header("*優先度: 低*", &priority_low_issues);
                message_block.add_text_block(text);
            }

            // add priority none issues
            let priority_none_issues = issues.priority_none_issues;
            if priority_none_issues.len() > 0 {
                let text = generate_text_with_header("*優先度: なし*", &priority_none_issues);
                message_block.add_text_block(text);
            }
        }
        Err(e) => {
            message_block.add_text_block(e.message);
        }
    }

    message_block
}

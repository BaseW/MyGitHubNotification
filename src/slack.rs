use crate::errors::GetIssueError;
use crate::models::{Issue, SlackMessageBlock, SlackMessageBlockText, SortedIssues};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageBlocks {
    #[serde(rename = "type")]
    blocks_type: String,
    blocks: Vec<SlackMessageBlock>,
}

impl Default for SlackMessageBlocks {
    fn default() -> Self {
        Self::new()
    }
}

impl SlackMessageBlocks {
    fn new() -> Self {
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

pub async fn notify_by_slack(webhook_url: String, message_blocks: SlackMessageBlocks) {
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
            println!("Notify by Slack Error: {err}");
        }
    }
}

fn generate_text_with_header(header: &str, issues: &Vec<Issue>) -> String {
    let mut text = String::new();
    text.push_str(format!("{header}\n").as_str());

    for issue in issues {
        let text_for_issue = generate_text_for_issue(issue);
        text.push_str(&text_for_issue);
    }
    text
}

fn generate_text_for_issue(issue: &Issue) -> String {
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
    format!(
        "- <{}|{}>(<{}|{}>): {}\n",
        issue_url, issue_title, issue_repository.html_url, issue_repository.name, issue_labels
    )
}

pub fn create_payload_for_slack(issues: Result<SortedIssues, GetIssueError>) -> SlackMessageBlocks {
    let mut message_block = SlackMessageBlocks::default();

    match issues {
        Ok(issues) => {
            // add mention to the channel
            message_block.add_text_block("<!channel>\n".to_string());
            message_block.add_header_block("タスク一覧".to_string());

            // add priority high issues
            let priority_high_issues = issues.priority_high_issues;
            if !priority_high_issues.is_empty() {
                let text = generate_text_with_header("*優先度: 高*", &priority_high_issues);
                message_block.add_text_block(text);
            }

            // add priority medium issues
            let priority_medium_issues = issues.priority_medium_issues;
            if !priority_medium_issues.is_empty() {
                let text = generate_text_with_header("*優先度: 中*", &priority_medium_issues);
                message_block.add_text_block(text);
            }

            // add priority low issues
            let priority_low_issues = issues.priority_low_issues;
            if !priority_low_issues.is_empty() {
                let text = generate_text_with_header("*優先度: 低*", &priority_low_issues);
                message_block.add_text_block(text);
            }

            // add priority none issues
            let priority_none_issues = issues.priority_none_issues;
            if !priority_none_issues.is_empty() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_header_block() {
        let mut slack_message_blocks = SlackMessageBlocks::default();
        assert_eq!(slack_message_blocks.blocks.len(), 0);

        // add header block
        slack_message_blocks.add_header_block("test".to_string());
        assert_eq!(slack_message_blocks.blocks.len(), 1);
        assert_eq!(slack_message_blocks.blocks[0].block_type, "header");
        assert_eq!(
            slack_message_blocks.blocks[0].text.as_ref().unwrap().text,
            "test"
        );
    }

    #[test]
    fn test_add_text_block() {
        let mut slack_message_blocks = SlackMessageBlocks::default();
        assert_eq!(slack_message_blocks.blocks.len(), 0);

        // add text block
        slack_message_blocks.add_text_block("test".to_string());
        assert_eq!(slack_message_blocks.blocks.len(), 1);
        assert_eq!(slack_message_blocks.blocks[0].block_type, "section");
        assert_eq!(
            slack_message_blocks.blocks[0].text.as_ref().unwrap().text,
            "test"
        );
    }

    #[tokio::test]
    async fn test_notify_by_slack() {
        use httpmock::prelude::*;

        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("content-type", "application/json");
            then.status(200);
        });

        let mock_webhook_url = format!("http://{}", server.address());
        let mock_message_blocks = SlackMessageBlocks::default();

        notify_by_slack(mock_webhook_url, mock_message_blocks).await;
        mock.assert();
    }

    #[test]
    fn test_generate_text_with_header() {
        use super::super::models::{Label, Repository};

        let issues = vec![Issue {
            html_url: "issue_html_url".to_string(),
            title: "title".to_string(),
            labels: Some(vec![Label {
                name: "label1".to_string(),
                id: 0,
            }]),
            repository: Repository {
                html_url: "repo_html_url".to_string(),
                name: "name".to_string(),
                id: 0,
            },
            body: None,
            id: 0,
            label_string: None,
            state: "open".to_string(),
        }];
        let text = generate_text_with_header("header", &issues);
        assert_eq!(
            text,
            "header".to_string()
                + "\n"
                + "- <issue_html_url|title>(<repo_html_url|name>): label1 \n"
        );
    }

    #[test]
    fn test_generate_text_for_issue() {
        use super::super::models::{Label, Repository};

        let issue = Issue {
            html_url: "issue_html_url".to_string(),
            title: "title".to_string(),
            labels: Some(vec![Label {
                name: "label1".to_string(),
                id: 0,
            }]),
            repository: Repository {
                html_url: "repo_html_url".to_string(),
                name: "name".to_string(),
                id: 0,
            },
            body: None,
            id: 0,
            label_string: None,
            state: "open".to_string(),
        };
        let text = generate_text_for_issue(&issue);
        assert_eq!(
            text,
            "- <issue_html_url|title>(<repo_html_url|name>): label1 \n"
        );
    }

    #[test]
    fn test_create_payload_for_slack() {
        use super::super::models::{Label, Repository};

        let mut issues = SortedIssues::default();
        issues.priority_high_issues.push(Issue {
            html_url: "issue_html_url".to_string(),
            title: "title".to_string(),
            labels: Some(vec![Label {
                name: "label1".to_string(),
                id: 0,
            }]),
            repository: Repository {
                html_url: "repo_html_url".to_string(),
                name: "name".to_string(),
                id: 0,
            },
            body: None,
            id: 0,
            label_string: None,
            state: "open".to_string(),
        });
        let payload = create_payload_for_slack(Ok(issues));
        assert_eq!(payload.blocks.len(), 3);
        assert_eq!(payload.blocks[0].block_type, "section");
        assert_eq!(
            payload.blocks[0].text.as_ref().unwrap().text,
            "<!channel>\n"
        );
        assert_eq!(payload.blocks[1].block_type, "header");
        assert_eq!(payload.blocks[1].text.as_ref().unwrap().text, "タスク一覧");
        assert_eq!(payload.blocks[2].block_type, "section");
        assert_eq!(
            payload.blocks[2].text.as_ref().unwrap().text,
            "*優先度: 高*".to_string()
                + "\n"
                + "- <issue_html_url|title>(<repo_html_url|name>): label1 \n"
        );
    }
}

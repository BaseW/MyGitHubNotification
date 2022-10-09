use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error;

#[derive(Debug, Serialize, Deserialize)]
struct Label {
    id: i64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    id: i64,
    title: String,
    url: String,
    state: String,
    body: Option<String>,
    labels: Option<Vec<Label>>,
}

#[derive(Debug, Clone)]
struct GetIssueError {
    message: String,
}

// implement for GetIssueError
impl std::fmt::Display for GetIssueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for GetIssueError {}

fn get_github_personal_access_token() -> String {
    // get token from environment variable
    let token = std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap();
    token
}

fn get_slack_webhook_url_from_env() -> String {
    // get token from environment variable
    let token = std::env::var("SLACK_WEBHOOK_URL").unwrap();
    token
}

async fn check_github_token_justification(token: &str) -> bool {
    // check if the token is justified
    let url = "https://api.github.com/user".to_string();
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let status = response.status();
    if status.is_success() {
        true
    } else {
        false
    }
}

async fn get_my_issues() -> Result<Vec<Issue>, GetIssueError> {
    let token = get_github_personal_access_token();
    // check github token justification
    let is_justified = check_github_token_justification(&token).await;
    if !is_justified {
        let error_message = "Github token is not justified";
        println!("{}", error_message);
        return Err(GetIssueError {
            message: error_message.to_string(),
        });
    }

    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/issues?filter=assigned&state=open&labels=Priority: High")
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    // if res is Ok, print "OK", otherwise print "Error"
    let issues = match res {
        Ok(res) => {
            println!("Fetch Issues OK");
            let parsed_issues = res.json::<Vec<Issue>>().await;
            match parsed_issues {
                Ok(issues) => issues,
                Err(e) => {
                    let error_message = format!("Parse Issues Error: {}", e);
                    println!("{}", error_message);
                    return Err(GetIssueError {
                        message: error_message,
                    });
                }
            }
        }
        Err(err) => {
            let error_message = format!("Fetch Issues Error: {}", err);
            println!("{}", error_message);
            return Err(GetIssueError {
                message: error_message,
            });
        }
    };
    Ok(issues)
}

async fn notify_by_slack(text: String) {
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

fn create_payload_for_slack(issues: Result<Vec<Issue>, GetIssueError>) -> String {
    let mut payload = String::new();

    match issues {
        Ok(issues) => {
            payload.push_str("@channel\n優先度が高いタスク一覧\n");
            if issues.len() == 0 {
                payload = "なし".to_string();
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

#[tokio::main]
async fn main() {
    let my_issues = get_my_issues().await;

    let payload = create_payload_for_slack(my_issues);

    // notify by slack
    notify_by_slack(payload).await;
}

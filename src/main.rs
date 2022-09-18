use serde::{Deserialize, Serialize};
use serde_json::json;

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

async fn get_my_issues() -> Vec<Issue> {
    let token = get_github_personal_access_token();
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
                    println!("Parse Issues Error: {}", e);
                    Vec::new()
                }
            }
        }
        Err(err) => {
            println!("Fetch Issues Error: {}", err);
            let empty_issues: Vec<Issue> = Vec::new();
            empty_issues
        }
    };
    issues
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

#[tokio::main]
async fn main() {
    let my_issues = get_my_issues().await;

    // print my_issues length
    println!("my_issues length: {}", my_issues.len());
    let text = format!("my_issues length: {}", my_issues.len());
    // print issues
    for issue in my_issues {
        println!("{:?}", issue);
    }

    // notify by slack
    notify_by_slack(text).await;
}

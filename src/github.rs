use crate::env::get_github_personal_access_token;
use crate::errors::GetIssueError;
use crate::models::{Issue, SortedIssues};

pub async fn get_my_issues() -> Result<Vec<Issue>, GetIssueError> {
    let token = get_github_personal_access_token();

    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/issues?filter=assigned&state=open")
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    // if res is Ok, print "OK", otherwise print "Error"
    let issues = match res {
        Ok(res) => {
            // if status is not 200, return error
            if res.status() != 200 {
                let error_message = format!("status code is not 200: {}", res.status());
                println!("{}", error_message);
                return Err(GetIssueError {
                    message: error_message,
                });
            }

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

pub fn sort_issues(
    issues: Result<Vec<Issue>, GetIssueError>,
) -> Result<SortedIssues, GetIssueError> {
    let mut priority_high_issues = Vec::new();
    let mut priority_medium_issues = Vec::new();
    let mut priority_low_issues = Vec::new();
    let mut priority_none_issues = Vec::new();
    match issues {
        Ok(issues) => {
            for issue in issues {
                let issue_labels = &issue.labels;
                match issue_labels {
                    Some(labels) => {
                        let mut is_priority_high = false;
                        let mut is_priority_medium = false;
                        let mut is_priority_low = false;
                        for label in labels {
                            if label.name == "Priority: High" {
                                is_priority_high = true;
                            }
                            if label.name == "Priority: Medium" {
                                is_priority_medium = true;
                            }
                            if label.name == "Priority: Low" {
                                is_priority_low = true;
                            }
                        }
                        if is_priority_high {
                            priority_high_issues.push(issue);
                        } else if is_priority_medium {
                            priority_medium_issues.push(issue);
                        } else if is_priority_low {
                            priority_low_issues.push(issue);
                        } else {
                            priority_none_issues.push(issue);
                        }
                    }
                    None => {
                        priority_none_issues.push(issue);
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e.message);
            return Err(GetIssueError { message: e.message });
        }
    }
    let sorted_issues = SortedIssues {
        priority_high_issues,
        priority_medium_issues,
        priority_low_issues,
        priority_none_issues,
    };
    Ok(sorted_issues)
}

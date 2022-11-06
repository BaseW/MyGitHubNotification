use crate::errors::GetIssueError;
use crate::models::{Issue, SortedIssues};

pub async fn get_my_issues(
    github_api_addr: String,
    token: String,
) -> Result<Vec<Issue>, GetIssueError> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "{}/issues?filter=assigned&state=open",
            github_api_addr
        ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_my_issues() {
        use super::super::models::{Issue, Repository};
        use httpmock::prelude::*;

        let mock_repo = Repository {
            id: 0,
            name: "test_repo".to_string(),
            html_url: "repo_url".to_string(),
        };
        let mock_issue = Issue {
            id: 1,
            title: "test".to_string(),
            body: Some("test".to_string()),
            labels: None,
            state: "open".to_string(),
            repository: mock_repo,
            html_url: "html_url".to_string(),
            label_string: None,
        };
        let mock_issues = vec![mock_issue];
        let token = String::from("token");

        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/issues")
                .header("User-Agent", "reqwest")
                .header("Accept", "application/vnd.github+json")
                .header("Authorization", format!("Bearer {}", token));
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&mock_issues);
        });
        let mock_api_addr = format!("http://{}", server.address());

        let issues = get_my_issues(mock_api_addr, token).await;
        mock.assert();
        assert!(issues.is_ok());
        assert_eq!(issues.unwrap().len(), 1);
    }

    #[test]
    fn test_sort_issues() {
        use super::super::models::{Issue, Repository};
        let mock_repo = Repository {
            id: 0,
            name: "test_repo".to_string(),
            html_url: "repo_url".to_string(),
        };
        let mock_issue = Issue {
            id: 1,
            title: "test".to_string(),
            body: Some("test".to_string()),
            labels: None,
            state: "open".to_string(),
            repository: mock_repo,
            html_url: "html_url".to_string(),
            label_string: None,
        };
        let mock_issues = vec![mock_issue];
        let issues = Ok(mock_issues);
        let sorted_issues = sort_issues(issues);
        assert!(sorted_issues.is_ok());
        let sorted_issues = sorted_issues.unwrap();
        assert_eq!(sorted_issues.priority_high_issues.len(), 0);
        assert_eq!(sorted_issues.priority_medium_issues.len(), 0);
        assert_eq!(sorted_issues.priority_low_issues.len(), 0);
        assert_eq!(sorted_issues.priority_none_issues.len(), 1);
    }
}

use crate::env::get_github_personal_access_token;
use crate::errors::GetIssueError;
use crate::models::Issue;

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

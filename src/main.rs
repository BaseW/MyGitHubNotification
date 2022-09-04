use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    id: i32,
    title: String,
    body: String,
}

fn get_github_personal_access_token() -> String {
    // get token from environment variable
    let token = std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap();
    token
}

// async fn get_my_issues() -> Vec<Issue> {
//     let token = get_github_personal_access_token();
//     let client = reqwest::Client::new();
//     let res = client
//         .get("https://api.github.com/issues")
//         .header("User-Agent", "reqwest")
//         .header("Accept", "application/vnd.github+json")
//         .header("Authorization", format!("Bearer {}", token))
//         .send()
//         .await;
//     // if res is Ok, print "OK", otherwise print "Error"
//     let issues = match res {
//         Ok(res) => {
//             println!("Fetch Issues OK");
//             let parsed_issues = res.json::<Vec<Issue>>().await;
//             match parsed_issues {
//                 Ok(issues) => issues,
//                 Err(e) => {
//                     println!("Parse Issues Error: {}", e);
//                     Vec::new()
//                 }
//             }
//         }
//         Err(err) => {
//             println!("Fetch Issues Error: {}", err);
//             let empty_issues: Vec<Issue> = Vec::new();
//             empty_issues
//         }
//     };
//     issues
// }

async fn get_my_issues_text() -> String {
    let token = get_github_personal_access_token();
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/issues")
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    // if res is Ok, print "OK", otherwise print "Error"
    let issues = match res {
        Ok(res) => {
            println!("Fetch Issues OK");
            let text = res.text().await;
            match text {
                Ok(text) => text,
                Err(e) => {
                    println!("Parse Issues Error: {}", e);
                    String::new()
                }
            }
        }
        Err(err) => {
            println!("Fetch Issues Error: {}", err);
            String::new()
        }
    };
    issues
}

#[tokio::main]
async fn main() {
    // let my_issues = get_my_issues().await;
    // // print issues
    // for issue in my_issues {
    //     println!("{:?}", issue);
    // }
    let my_issues_text = get_my_issues_text().await;
    // print my issues text
    println!("{}", my_issues_text);
}

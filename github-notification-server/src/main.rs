use axum::{response::Html, routing::get, Router};
use github_notification::{
    env::{get_github_personal_access_token, get_slack_webhook_url_from_env},
    github::{get_my_issues, sort_issues},
    sentry::initialize_sentry,
    slack::{create_payload_for_slack, notify_by_slack},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/get-issues", get(get_issues_handler));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// receive request from slack
// respond with text
async fn get_issues_handler() -> String {
    let _guard = initialize_sentry();

    let token = get_github_personal_access_token();
    let webhook_url = get_slack_webhook_url_from_env();

    let github_api_addr = "https://api.github.com".to_string();
    let my_issues = get_my_issues(github_api_addr, token).await;
    let sorted_issues = sort_issues(my_issues);
    let payload = create_payload_for_slack(sorted_issues);

    // notify by slack
    notify_by_slack(webhook_url, payload).await;
    String::from("ok")
}

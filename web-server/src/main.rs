//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-hello-world
//! ```

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use github_notification::{sentry::initialize_sentry, github::{get_my_issues, sort_issues}, slack::{create_payload_for_slack, notify_by_slack}, env::{get_github_personal_access_token, get_slack_webhook_url_from_env}};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    let _guard = initialize_sentry();

    let token = get_github_personal_access_token();
    let webhook_url = get_slack_webhook_url_from_env();

    let github_api_addr = "https://api.github.com".to_string();
    let my_issues = get_my_issues(github_api_addr, token).await;
    let sorted_issues = sort_issues(my_issues);
    let payload = create_payload_for_slack(sorted_issues);

    // notify by slack
    notify_by_slack(webhook_url, payload).await;
    Html("<h1>Hello, World!</h1>")
}

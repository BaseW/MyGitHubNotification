use axum::{
    routing::{get, post},
    Router,
};
use github_notification::sentry::initialize_sentry;
use github_notification_server::handlers::{
    health_check::health_check_handler, notification::create_notification_handler,
};
use github_notification_server::logger::access_log_on_request;
use std::net::SocketAddr;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let _guard = initialize_sentry();
    tracing_subscriber::fmt::init();
    // build our application with a route
    let app = Router::new()
        .route("/", get(health_check_handler))
        .route("/create-notification", post(create_notification_handler))
        .layer(
            ServiceBuilder::new()
              .layer(axum::middleware::from_fn(access_log_on_request))
        );

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

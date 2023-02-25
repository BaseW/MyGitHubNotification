use axum::{
    routing::{get, post},
    Router,
};
use github_notification::sentry::initialize_sentry;
use github_notification_server::handlers::{
    health_check::health_check_handler, notification::create_notification_handler,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let _guard = initialize_sentry();
    tracing_subscriber::fmt::init();
    // build our application with a route
    let app = Router::new()
        .route("/", get(health_check_handler))
        .route("/create-notification", post(create_notification_handler))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

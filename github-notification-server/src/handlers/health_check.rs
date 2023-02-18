use axum::response::Html;

pub async fn health_check_handler() -> Html<&'static str> {
    Html("<h1>HealthCheck OK</h1>")
}

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn access_log_on_request<B: std::fmt::Debug>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    println!(
        "{} {} {:?} {:?}",
        req.method(),
        req.uri(),
        req.headers(),
        req.body()
    );
    Ok(next.run(req).await)
}

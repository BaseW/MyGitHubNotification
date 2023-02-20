use axum::{http::{StatusCode, Request}, response::Response, middleware::Next};

pub async fn access_log_on_request<B>(
  req: Request<B>,
  next: Next<B>,
) -> Result<Response, StatusCode> {
  println!("{} {}", req.method(), req.uri());
  Ok(next.run(req).await)
}

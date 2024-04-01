use anyhow::Result;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

fn get_token(headers: &HeaderMap) -> Result<&str> {
    let token = headers
        .get("auth")
        .ok_or(anyhow::anyhow!("missing authorization header"))?
        .to_str()?;
    Ok(token)
}

pub async fn auth(
    State(token): State<String>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Ok(t) if token == t => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

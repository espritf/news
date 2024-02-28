use anyhow::Result;
use axum::{
    http::{StatusCode, HeaderMap},
    extract::Request,
    middleware::Next,
    response::Response
};

pub fn token_is_valid(token: &str) -> bool {
    let valid_token = std::env::var("NEWS_API_TOKEN").unwrap();
    token == valid_token
}

pub fn get_token(headers: &HeaderMap) -> Result<&str> {
    let token = headers.get("auth")
        .ok_or(anyhow::anyhow!("missing authorization header"))?
        .to_str()?;
    Ok(token)
}

pub async fn auth(headers: HeaderMap, request: Request, next: Next) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Ok(token) if token_is_valid(token) => {
            let response = next.run(request).await;
            Ok(response)
        },
        _ => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

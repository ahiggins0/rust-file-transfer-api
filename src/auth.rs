use axum::{http::{Request, StatusCode}, response::{Response,IntoResponse}};
use axum::middleware::{Next};
use base64::decode;
use std::env;

pub async fn auth_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    println!("Received a request to {}", req.uri());
    println!("Detailed info: {:?}", req.extensions().get::<std::net::SocketAddr>());

    let user = env::var("BASIC_AUTH_USER").expect("BASIC_AUTH_USER environment variable must be set");
    let pass = env::var("BASIC_AUTH_PASSWORD").expect("BASIC_AUTH_PASSWORD environment variable must be set");

    // Extract the `Authorization` header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(credentials) = auth_str.strip_prefix("Basic ") {
                if let Ok(decoded) = decode(credentials) {
                    if let Ok(decoded_str) = String::from_utf8(decoded) {
                        // Validate username and password (e.g., "username:password")
                        if decoded_str == format!("{}:{}", user, pass) {
                            return next.run(req).await; // Proceed to the next handler
                        }
                    }
                }
            }
        }
    }

    // Return 401 Unauthorized if authentication fails
    (
        StatusCode::UNAUTHORIZED,
        [("WWW-Authenticate", "Basic realm=\"example\"")],
        "Unauthorized",
    )
        .into_response()
}

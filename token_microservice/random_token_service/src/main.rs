use axum::{
    routing::get,
    Router,
    Json,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

async fn generate_token() -> Json<TokenResponse> {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)  // Generate a token with 30 characters
        .map(char::from)
        .collect();

    Json(TokenResponse { token })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/token", get(generate_token));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

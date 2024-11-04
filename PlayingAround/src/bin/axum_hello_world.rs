use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::IntoResponse,
};

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "HI LISA")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
use std::net::Ipv4Addr;

use anyhow::Result;
use axum::{Router, routing::get};
use handlers::{handle_about, handle_home, handle_list, handle_snake};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod handlers;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handle_home))
        .route("/about", get(handle_about))
        .route("/list", get(handle_list))
        .route("/snake", get(handle_snake))
        .fallback_service(ServeDir::new("static").precompressed_gzip());

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3000)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

use std::net::Ipv4Addr;

use crate::views::Page;
use anyhow::Result;
use axum::{Router, routing::get};
use handlers::{handle_about, handle_home, handle_projects, handle_snake};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

mod handlers;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Router::new()
        .route(Page::Home.get_ref(), get(handle_home))
        .route(Page::About.get_ref(), get(handle_about))
        .route(Page::Projects.get_ref(), get(handle_projects))
        .route(Page::Snake.get_ref(), get(handle_snake))
        .fallback_service(ServeDir::new("static").precompressed_gzip());

    if cfg!(debug_assertions) {
        app = app.layer(LiveReloadLayer::new());
    }

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3000)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

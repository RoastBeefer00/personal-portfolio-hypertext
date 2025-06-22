use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use hypertext::prelude::*;

use crate::views::{Document, Nav, Page, about, home, projects, snake};

fn maybe_document<R: Renderable>(
    HxRequest(is_hx_request): HxRequest,
    selected: Page,
    children: R,
) -> impl IntoResponse {
    rsx! {
        @if is_hx_request {
            <Nav selected=selected oob=true />
            (children)
        } @else {
            <Document selected=selected>
                (children)
            </Document>
        }
    }
}

pub async fn handle_home(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, Page::Home, home())
}

pub async fn handle_about(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, Page::About, about())
}

pub async fn handle_projects(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, Page::Projects, projects())
}

pub async fn handle_snake(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, Page::Snake, snake())
}

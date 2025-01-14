use axum::debug_handler;
use axum::response::Html;
use std::fs;

const INDEX_FILE: &str = "static/index.html";

#[debug_handler]
pub async fn serve_index() -> Html<String> {
    let content = fs::read_to_string(INDEX_FILE).expect("Failed to read index.html");
    axum::response::Html(content)
}

use crate::api::http_service;

mod api;

#[tokio::main]
async fn main() {
    http_service().await;
}

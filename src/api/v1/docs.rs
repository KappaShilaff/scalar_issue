use aide::axum::IntoApiResponse;
use aide::openapi::OpenApi;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::ops::Deref;
use std::sync::Arc;

pub async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api.deref()).into_response()
}

pub async fn yaml_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    serde_yaml::to_string(api.deref()).unwrap().into_response()
}

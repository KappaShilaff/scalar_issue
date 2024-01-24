use crate::api::get_open_api;
use crate::api::v1::docs::serve_docs;
use aide::axum::routing::get;
use aide::axum::ApiRouter;
use aide::scalar::Scalar;
use axum::{Extension, Router};
use std::sync::Arc;

mod docs;
pub mod issue;

pub fn router() -> Router {
    let mut api = get_open_api("v1", "http://localhost:9000");

    ApiRouter::new()
        .nest("/scalar", issue::router())
        .route("/docs", Scalar::new("/v1/api.json").axum_route())
        .api_route_with("/swagger.yaml", get(docs::yaml_docs), |op| {
            op.tag("swagger")
        })
        .route("/api.json", get(serve_docs))
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
}

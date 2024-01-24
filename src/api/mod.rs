use aide::openapi::{Info, OpenApi, Server};
use axum::Router;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod v1;

pub fn router() -> Router {
    Router::new().nest("/v1", v1::router())
}

pub async fn http_service() {
    aide::gen::extract_schemas(true);

    let router = router().layer(ServiceBuilder::new().layer(CorsLayer::permissive()));

    let listener = tokio::net::TcpListener::bind::<&SocketAddr>(&"0.0.0.0:9000".parse().unwrap())
        .await
        .unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

fn get_open_api(version: &str, url: &str) -> OpenApi {
    OpenApi {
        info: Info {
            description: Some("Issue".to_string()),
            ..Info::default()
        },
        servers: vec![Server {
            url: format!("{}/{}", url, version),
            description: Some("test".to_string()),
            variables: Default::default(),
            extensions: Default::default(),
        }],
        ..OpenApi::default()
    }
}

pub fn example_issue() -> &'static str {
    "1337"
}

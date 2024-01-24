use aide::axum::routing::post;
use aide::axum::ApiRouter;
use aide::transform::TransformPathItem;
use axum::Json;

use crate::api::v1::issue::requests::IssueRequest;
use crate::api::v1::issue::responses::IssueResponse;

pub mod requests;
pub mod responses;

pub fn router() -> ApiRouter {
    ApiRouter::new().api_route_with("/issue", post(issue), tag)
}

pub async fn issue(Json(_input): Json<IssueRequest>) -> Json<IssueResponse> {
    Json(IssueResponse {
        issue_id: "issue".to_string(),
    })
}

fn tag(op: TransformPathItem) -> TransformPathItem {
    op.tag("issue")
}

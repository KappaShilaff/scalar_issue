use crate::api::example_issue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueResponse {
    #[schemars(example = "example_issue")]
    pub issue_id: String,
}

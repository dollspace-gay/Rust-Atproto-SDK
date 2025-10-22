//! Generated code for app.bsky.unspecced.getSuggestedUsersSkeleton
//!
//! Get a skeleton of suggested users. Intended to be called and hydrated by app.bsky.unspecced.getSuggestedUsers

use crate::xrpc::{XrpcClient, XrpcRequest, XrpcResponse, XrpcError};
use serde::{Deserialize, Serialize};

/// Query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    /// DID of the account making the request (not included for public/unauthenticated queries).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<crate::types::Did>,
    /// Category of users to get suggestions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Response output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub dids: serde_json::Value,
}

/// Get a skeleton of suggested users. Intended to be called and hydrated by app.bsky.unspecced.getSuggestedUsers
pub async fn get_suggested_users_skeleton(
    client: &impl XrpcClient,
    params: QueryParams,
) -> Result<XrpcResponse<Output>, XrpcError> {
    let mut req = XrpcRequest::query("app.bsky.unspecced.getSuggestedUsersSkeleton");

    // Add query parameters
    let params_json = serde_json::to_value(&params)
        .map_err(XrpcError::Serialization)?;

    if let Some(obj) = params_json.as_object() {
        for (key, value) in obj {
            if let Some(s) = value.as_str() {
                req.params.insert(key.clone(), s.to_string());
            } else {
                req.params.insert(key.clone(), value.to_string());
            }
        }
    }

    client.request(req).await
}

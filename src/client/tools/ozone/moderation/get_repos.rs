//! Generated code for tools.ozone.moderation.getRepos
//!
//! Get details about some repositories.

use crate::xrpc::{XrpcClient, XrpcRequest, XrpcResponse, XrpcError};
use serde::{Deserialize, Serialize};

/// Query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    pub dids: serde_json::Value,
}

/// Response output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub repos: serde_json::Value,
}

/// Get details about some repositories.
pub async fn get_repos(
    client: &impl XrpcClient,
    params: QueryParams,
) -> Result<XrpcResponse<Output>, XrpcError> {
    let mut req = XrpcRequest::query("tools.ozone.moderation.getRepos");

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

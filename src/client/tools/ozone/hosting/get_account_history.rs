//! Generated code for tools.ozone.hosting.getAccountHistory
//!
//! Get account history, e.g. log of updated email addresses or other identity information.

use crate::xrpc::{XrpcClient, XrpcRequest, XrpcResponse, XrpcError};
use serde::{Deserialize, Serialize};

/// Query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub did: crate::types::Did,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Response output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub events: serde_json::Value,
}

/// Get account history, e.g. log of updated email addresses or other identity information.
pub async fn get_account_history(
    client: &impl XrpcClient,
    params: QueryParams,
) -> Result<XrpcResponse<Output>, XrpcError> {
    let mut req = XrpcRequest::query("tools.ozone.hosting.getAccountHistory");

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

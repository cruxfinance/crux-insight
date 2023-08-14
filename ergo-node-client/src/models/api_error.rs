/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code
    #[serde(rename = "error")]
    pub error: i32,
    /// String error code
    #[serde(rename = "reason")]
    pub reason: String,
    /// Detailed error description
    #[serde(rename = "detail", deserialize_with = "Option::deserialize")]
    pub detail: Option<String>,
}

impl ApiError {
    pub fn new(error: i32, reason: String, detail: Option<String>) -> ApiError {
        ApiError {
            error,
            reason,
            detail,
        }
    }
}


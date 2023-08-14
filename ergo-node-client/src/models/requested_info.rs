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
pub struct RequestedInfo {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// How many times we checked for modifier delivery status
    #[serde(rename = "checks")]
    pub checks: i32,
}

impl RequestedInfo {
    pub fn new(checks: i32) -> RequestedInfo {
        RequestedInfo {
            address: None,
            version: None,
            checks,
        }
    }
}



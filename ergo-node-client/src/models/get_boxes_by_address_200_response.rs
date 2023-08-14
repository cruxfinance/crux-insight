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
pub struct GetBoxesByAddress200Response {
    /// Array of boxes
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::IndexedErgoBox>>,
    /// Total number of retreived boxes
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl GetBoxesByAddress200Response {
    pub fn new() -> GetBoxesByAddress200Response {
        GetBoxesByAddress200Response {
            items: None,
            total: None,
        }
    }
}



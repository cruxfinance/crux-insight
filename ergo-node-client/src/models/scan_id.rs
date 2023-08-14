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
pub struct ScanId {
    #[serde(rename = "scanId", skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<i32>,
}

impl ScanId {
    pub fn new() -> ScanId {
        ScanId {
            scan_id: None,
        }
    }
}



/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// ScanIdsBox : Ergo box with associated scans (their respective identifiers)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScanIdsBox {
    #[serde(rename = "scanIds")]
    pub scan_ids: Vec<i32>,
    #[serde(rename = "box")]
    pub r#box: Box<crate::models::ErgoTransactionOutput>,
}

impl ScanIdsBox {
    /// Ergo box with associated scans (their respective identifiers)
    pub fn new(scan_ids: Vec<i32>, r#box: crate::models::ErgoTransactionOutput) -> ScanIdsBox {
        ScanIdsBox {
            scan_ids,
            r#box: Box::new(r#box),
        }
    }
}



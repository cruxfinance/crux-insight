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
pub struct ErgoTreeObject {
    /// serialized Ergo tree
    #[serde(rename = "tree", skip_serializing_if = "Option::is_none")]
    pub tree: Option<String>,
}

impl ErgoTreeObject {
    pub fn new() -> ErgoTreeObject {
        ErgoTreeObject {
            tree: None,
        }
    }
}



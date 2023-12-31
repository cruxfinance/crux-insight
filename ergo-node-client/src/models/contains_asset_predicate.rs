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
pub struct ContainsAssetPredicate {
    #[serde(rename = "predicate")]
    pub predicate: String,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
}

impl ContainsAssetPredicate {
    pub fn new(predicate: String) -> ContainsAssetPredicate {
        ContainsAssetPredicate {
            predicate,
            asset_id: None,
        }
    }
}



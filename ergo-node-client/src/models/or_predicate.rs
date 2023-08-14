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
pub struct OrPredicate {
    #[serde(rename = "predicate")]
    pub predicate: String,
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<crate::models::ScanningPredicate>>,
}

impl OrPredicate {
    pub fn new(predicate: String) -> OrPredicate {
        OrPredicate {
            predicate,
            args: None,
        }
    }
}


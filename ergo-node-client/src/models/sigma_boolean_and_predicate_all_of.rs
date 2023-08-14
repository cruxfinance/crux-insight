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
pub struct SigmaBooleanAndPredicateAllOf {
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<crate::models::SigmaBoolean>>,
}

impl SigmaBooleanAndPredicateAllOf {
    pub fn new() -> SigmaBooleanAndPredicateAllOf {
        SigmaBooleanAndPredicateAllOf {
            args: None,
        }
    }
}


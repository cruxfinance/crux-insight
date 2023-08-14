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
pub struct ExecuteScript {
    /// Sigma script to be executed
    #[serde(rename = "script")]
    pub script: String,
    /// Environment for compiler
    #[serde(rename = "namedConstants", deserialize_with = "Option::deserialize")]
    pub named_constants: Option<serde_json::Value>,
    #[serde(rename = "context")]
    pub context: crate::models::ErgoLikeContext,
}

impl ExecuteScript {
    pub fn new(script: String, named_constants: Option<serde_json::Value>, context: crate::models::ErgoLikeContext) -> ExecuteScript {
        ExecuteScript {
            script,
            named_constants,
            context,
        }
    }
}


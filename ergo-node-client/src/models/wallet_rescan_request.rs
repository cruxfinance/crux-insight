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
pub struct WalletRescanRequest {
    #[serde(rename = "fromHeight")]
    pub from_height: i32,
}

impl WalletRescanRequest {
    pub fn new(from_height: i32) -> WalletRescanRequest {
        WalletRescanRequest {
            from_height,
        }
    }
}



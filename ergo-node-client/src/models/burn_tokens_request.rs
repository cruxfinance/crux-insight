/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// BurnTokensRequest : Request for burning tokens in wallet



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BurnTokensRequest {
    /// Assets list to burn in the transaction
    #[serde(rename = "assetsToBurn")]
    pub assets_to_burn: Vec<crate::models::Asset>,
}

impl BurnTokensRequest {
    /// Request for burning tokens in wallet
    pub fn new(assets_to_burn: Vec<crate::models::Asset>) -> BurnTokensRequest {
        BurnTokensRequest {
            assets_to_burn,
        }
    }
}


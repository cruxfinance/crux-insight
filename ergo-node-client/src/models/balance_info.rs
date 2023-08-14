/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// BalanceInfo : Balance information



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BalanceInfo {
    /// Balance of nanoERGs
    #[serde(rename = "nanoErgs")]
    pub nano_ergs: i64,
    /// Balance of tokens
    #[serde(rename = "tokens")]
    pub tokens: Vec<crate::models::BalanceInfoTokensInner>,
}

impl BalanceInfo {
    /// Balance information
    pub fn new(nano_ergs: i64, tokens: Vec<crate::models::BalanceInfoTokensInner>) -> BalanceInfo {
        BalanceInfo {
            nano_ergs,
            tokens,
        }
    }
}



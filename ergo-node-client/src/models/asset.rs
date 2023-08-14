/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// Asset : Token detail in the transaction



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Asset {
    /// Base16-encoded 32 byte digest
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// Amount of the token
    #[serde(rename = "amount")]
    pub amount: i64,
}

impl Asset {
    /// Token detail in the transaction
    pub fn new(token_id: String, amount: i64) -> Asset {
        Asset {
            token_id,
            amount,
        }
    }
}


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
pub struct BlockTransactions {
    /// Base16-encoded 32 byte modifier id
    #[serde(rename = "headerId")]
    pub header_id: String,
    /// Ergo transaction objects
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::ErgoTransaction>,
    /// Size in bytes
    #[serde(rename = "size")]
    pub size: i32,
}

impl BlockTransactions {
    pub fn new(header_id: String, transactions: Vec<crate::models::ErgoTransaction>, size: i32) -> BlockTransactions {
        BlockTransactions {
            header_id,
            transactions,
            size,
        }
    }
}


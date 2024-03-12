/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// FullBlock : Block with header and transactions

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FullBlock {
    #[serde(rename = "header")]
    pub header: Box<crate::models::BlockHeader>,
    #[serde(rename = "blockTransactions")]
    pub block_transactions: Box<crate::models::BlockTransactions>,
    #[serde(rename = "adProofs")]
    pub ad_proofs: Box<Option<crate::models::BlockAdProofs>>,
    #[serde(rename = "extension")]
    pub extension: Box<crate::models::Extension>,
    /// Size in bytes
    #[serde(rename = "size")]
    pub size: i32,
}

impl FullBlock {
    /// Block with header and transactions
    pub fn new(
        header: crate::models::BlockHeader,
        block_transactions: crate::models::BlockTransactions,
        ad_proofs: Option<crate::models::BlockAdProofs>,
        extension: crate::models::Extension,
        size: i32,
    ) -> FullBlock {
        FullBlock {
            header: Box::new(header),
            block_transactions: Box::new(block_transactions),
            ad_proofs: Box::new(ad_proofs),
            extension: Box::new(extension),
            size,
        }
    }
}

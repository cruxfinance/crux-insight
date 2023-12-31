/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// SigmaHeader : Block header format used for sigma ErgoLikeContext



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SigmaHeader {
    /// Base16-encoded 32 byte modifier id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Basic timestamp definition
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Ergo blockchain protocol version
    #[serde(rename = "version")]
    pub version: i32,
    /// Base16-encoded 32 byte digest
    #[serde(rename = "adProofsRoot")]
    pub ad_proofs_root: String,
    /// Base16-encoded 32 byte modifier id
    #[serde(rename = "adProofsId", skip_serializing_if = "Option::is_none")]
    pub ad_proofs_id: Option<String>,
    #[serde(rename = "stateRoot")]
    pub state_root: Box<crate::models::AvlTreeData>,
    /// Base16-encoded 32 byte digest
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: String,
    /// Base16-encoded 32 byte modifier id
    #[serde(rename = "transactionsId", skip_serializing_if = "Option::is_none")]
    pub transactions_id: Option<String>,
    #[serde(rename = "nBits")]
    pub n_bits: i64,
    /// Base16-encoded 32 byte digest
    #[serde(rename = "extensionHash")]
    pub extension_hash: String,
    /// Base16-encoded 32 byte digest
    #[serde(rename = "extensionRoot", skip_serializing_if = "Option::is_none")]
    pub extension_root: Option<String>,
    /// Base16-encoded 32 byte modifier id
    #[serde(rename = "extensionId", skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Base16-encoded 32 byte modifier id
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "powSolutions", skip_serializing_if = "Option::is_none")]
    pub pow_solutions: Option<Box<crate::models::PowSolutions>>,
    /// Base16-encoded votes for a soft-fork and parameters
    #[serde(rename = "votes")]
    pub votes: String,
    #[serde(rename = "minerPk", skip_serializing_if = "Option::is_none")]
    pub miner_pk: Option<String>,
    #[serde(rename = "powOnetimePk", skip_serializing_if = "Option::is_none")]
    pub pow_onetime_pk: Option<String>,
    /// Base16-encoded 32 byte digest
    #[serde(rename = "powNonce", skip_serializing_if = "Option::is_none")]
    pub pow_nonce: Option<String>,
    /// sigma.BigInt
    #[serde(rename = "powDistance", skip_serializing_if = "Option::is_none")]
    pub pow_distance: Option<f32>,
}

impl SigmaHeader {
    /// Block header format used for sigma ErgoLikeContext
    pub fn new(timestamp: i64, version: i32, ad_proofs_root: String, state_root: crate::models::AvlTreeData, transactions_root: String, n_bits: i64, extension_hash: String, height: i32, parent_id: String, votes: String) -> SigmaHeader {
        SigmaHeader {
            id: None,
            timestamp,
            version,
            ad_proofs_root,
            ad_proofs_id: None,
            state_root: Box::new(state_root),
            transactions_root,
            transactions_id: None,
            n_bits,
            extension_hash,
            extension_root: None,
            extension_id: None,
            height,
            size: None,
            parent_id,
            pow_solutions: None,
            votes,
            miner_pk: None,
            pow_onetime_pk: None,
            pow_nonce: None,
            pow_distance: None,
        }
    }
}



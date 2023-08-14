/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// MerkleProof : Merkle proof for a leaf, which is an array of bytes (e.g. a transaction identifier)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Base16-encoded Merkle tree leaf bytes
    #[serde(rename = "leaf")]
    pub leaf: String,
    #[serde(rename = "levels")]
    pub levels: Vec<Vec<crate::models::MerkleProofLevelsInnerInner>>,
}

impl MerkleProof {
    /// Merkle proof for a leaf, which is an array of bytes (e.g. a transaction identifier)
    pub fn new(leaf: String, levels: Vec<Vec<crate::models::MerkleProofLevelsInnerInner>>) -> MerkleProof {
        MerkleProof {
            leaf,
            levels,
        }
    }
}


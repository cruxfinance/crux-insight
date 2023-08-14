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
pub struct ErgoTransactionDataInput {
    /// Base16-encoded transaction box id bytes. Should be 32 bytes long
    #[serde(rename = "boxId")]
    pub box_id: String,
}

impl ErgoTransactionDataInput {
    pub fn new(box_id: String) -> ErgoTransactionDataInput {
        ErgoTransactionDataInput {
            box_id,
        }
    }
}


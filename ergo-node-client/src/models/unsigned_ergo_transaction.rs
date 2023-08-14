/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// UnsignedErgoTransaction : Unsigned Ergo transaction



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnsignedErgoTransaction {
    /// Base16-encoded transaction id bytes
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unsigned inputs of the transaction
    #[serde(rename = "inputs")]
    pub inputs: Vec<crate::models::ErgoTransactionUnsignedInput>,
    /// Data inputs of the transaction
    #[serde(rename = "dataInputs")]
    pub data_inputs: Vec<crate::models::ErgoTransactionDataInput>,
    /// Outputs of the transaction
    #[serde(rename = "outputs")]
    pub outputs: Vec<crate::models::ErgoTransactionOutput>,
}

impl UnsignedErgoTransaction {
    /// Unsigned Ergo transaction
    pub fn new(inputs: Vec<crate::models::ErgoTransactionUnsignedInput>, data_inputs: Vec<crate::models::ErgoTransactionDataInput>, outputs: Vec<crate::models::ErgoTransactionOutput>) -> UnsignedErgoTransaction {
        UnsignedErgoTransaction {
            id: None,
            inputs,
            data_inputs,
            outputs,
        }
    }
}


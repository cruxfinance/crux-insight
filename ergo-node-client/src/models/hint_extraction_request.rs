/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// HintExtractionRequest : request to extract prover hints from a transaction



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HintExtractionRequest {
    #[serde(rename = "tx")]
    pub tx: Box<crate::models::ErgoTransaction>,
    /// Real signers of the transaction
    #[serde(rename = "real")]
    pub real: Vec<crate::models::SigmaBoolean>,
    /// Simulated signers of the transaction
    #[serde(rename = "simulated")]
    pub simulated: Vec<crate::models::SigmaBoolean>,
    /// Optional list of inputs to be used in serialized form
    #[serde(rename = "inputsRaw", skip_serializing_if = "Option::is_none")]
    pub inputs_raw: Option<Vec<String>>,
    /// Optional list of inputs to be used in serialized form
    #[serde(rename = "dataInputsRaw", skip_serializing_if = "Option::is_none")]
    pub data_inputs_raw: Option<Vec<String>>,
}

impl HintExtractionRequest {
    /// request to extract prover hints from a transaction
    pub fn new(tx: crate::models::ErgoTransaction, real: Vec<crate::models::SigmaBoolean>, simulated: Vec<crate::models::SigmaBoolean>) -> HintExtractionRequest {
        HintExtractionRequest {
            tx: Box::new(tx),
            real,
            simulated,
            inputs_raw: None,
            data_inputs_raw: None,
        }
    }
}


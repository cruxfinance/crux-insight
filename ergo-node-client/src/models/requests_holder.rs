/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// RequestsHolder : Holds many transaction requests and transaction fee



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestsHolder {
    /// Sequence of transaction requests
    #[serde(rename = "requests")]
    pub requests: Vec<crate::models::RequestsHolderRequestsInner>,
    /// Transaction fee
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<i64>,
    /// List of inputs to be used in serialized form
    #[serde(rename = "inputsRaw", skip_serializing_if = "Option::is_none")]
    pub inputs_raw: Option<Vec<String>>,
    /// List of data inputs to be used in serialized form
    #[serde(rename = "dataInputsRaw", skip_serializing_if = "Option::is_none")]
    pub data_inputs_raw: Option<Vec<String>>,
}

impl RequestsHolder {
    /// Holds many transaction requests and transaction fee
    pub fn new(requests: Vec<crate::models::RequestsHolderRequestsInner>) -> RequestsHolder {
        RequestsHolder {
            requests,
            fee: None,
            inputs_raw: None,
            data_inputs_raw: None,
        }
    }
}



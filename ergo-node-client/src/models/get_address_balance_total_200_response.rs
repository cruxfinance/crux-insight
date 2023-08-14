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
pub struct GetAddressBalanceTotal200Response {
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<Box<crate::models::BalanceInfo>>,
    #[serde(rename = "unconfirmed", skip_serializing_if = "Option::is_none")]
    pub unconfirmed: Option<Box<crate::models::BalanceInfo>>,
}

impl GetAddressBalanceTotal200Response {
    pub fn new() -> GetAddressBalanceTotal200Response {
        GetAddressBalanceTotal200Response {
            confirmed: None,
            unconfirmed: None,
        }
    }
}


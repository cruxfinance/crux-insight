/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// AddressHolder : Holds encoded ErgoAddress



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressHolder {
    /// Encoded Ergo Address
    #[serde(rename = "address")]
    pub address: String,
}

impl AddressHolder {
    /// Holds encoded ErgoAddress
    pub fn new(address: String) -> AddressHolder {
        AddressHolder {
            address,
        }
    }
}



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
pub struct DeriveNextKeyResult {
    /// Derivation path of the resulted secret
    #[serde(rename = "derivationPath")]
    pub derivation_path: String,
    /// Encoded Ergo Address
    #[serde(rename = "address")]
    pub address: String,
}

impl DeriveNextKeyResult {
    pub fn new(derivation_path: String, address: String) -> DeriveNextKeyResult {
        DeriveNextKeyResult {
            derivation_path,
            address,
        }
    }
}



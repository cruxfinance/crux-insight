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
pub struct BlacklistedPeers {
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
}

impl BlacklistedPeers {
    pub fn new(addresses: Vec<String>) -> BlacklistedPeers {
        BlacklistedPeers {
            addresses,
        }
    }
}



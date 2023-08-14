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
pub struct ConnectedPeer {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Basic timestamp definition
    #[serde(rename = "lastMessage", skip_serializing_if = "Option::is_none")]
    pub last_message: Option<i64>,
}

impl ConnectedPeer {
    pub fn new(address: String) -> ConnectedPeer {
        ConnectedPeer {
            address,
            version: None,
            last_message: None,
        }
    }
}



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
pub struct PassphraseMatch {
    /// true if passphrase matches wallet, false otherwise
    #[serde(rename = "matched")]
    pub matched: bool,
}

impl PassphraseMatch {
    pub fn new(matched: bool) -> PassphraseMatch {
        PassphraseMatch {
            matched,
        }
    }
}



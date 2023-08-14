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
pub struct CheckWallet {
    /// Mnemonic seed (optional)
    #[serde(rename = "mnemonic")]
    pub mnemonic: String,
    /// Optional pass to password-protect mnemonic seed
    #[serde(rename = "mnemonicPass", skip_serializing_if = "Option::is_none")]
    pub mnemonic_pass: Option<String>,
}

impl CheckWallet {
    pub fn new(mnemonic: String) -> CheckWallet {
        CheckWallet {
            mnemonic,
            mnemonic_pass: None,
        }
    }
}


/*
 * Ergo Node API
 *
 * API docs for Ergo Node. Models are shared between all Ergo products
 *
 * The version of the OpenAPI document: 5.0.12
 * Contact: ergoplatform@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// BoxesRequestHolder : Holds request for wallet boxes



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BoxesRequestHolder {
    /// Target assets
    #[serde(rename = "targetAssets")]
    pub target_assets: Vec<Vec<crate::models::BoxesRequestHolderTargetAssetsInnerInner>>,
    /// Target balance
    #[serde(rename = "targetBalance")]
    pub target_balance: i64,
}

impl BoxesRequestHolder {
    /// Holds request for wallet boxes
    pub fn new(target_assets: Vec<Vec<crate::models::BoxesRequestHolderTargetAssetsInnerInner>>, target_balance: i64) -> BoxesRequestHolder {
        BoxesRequestHolder {
            target_assets,
            target_balance,
        }
    }
}


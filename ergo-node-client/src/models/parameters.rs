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
pub struct Parameters {
    /// Height when current parameters were considered(not actual height). Can be '0' if state is empty
    #[serde(rename = "height")]
    pub height: i32,
    /// Storage fee coefficient (per byte per storage period ~4 years)
    #[serde(rename = "storageFeeFactor")]
    pub storage_fee_factor: i32,
    /// Minimum value per byte of an output
    #[serde(rename = "minValuePerByte")]
    pub min_value_per_byte: i32,
    /// Maximum block size (in bytes)
    #[serde(rename = "maxBlockSize")]
    pub max_block_size: i32,
    /// Maximum cumulative computational cost of input scripts in block transactions
    #[serde(rename = "maxBlockCost")]
    pub max_block_cost: i32,
    /// Ergo blockchain protocol version
    #[serde(rename = "blockVersion")]
    pub block_version: i32,
    /// Validation cost of a single token
    #[serde(rename = "tokenAccessCost")]
    pub token_access_cost: i32,
    /// Validation cost per one transaction input
    #[serde(rename = "inputCost")]
    pub input_cost: i32,
    /// Validation cost per one data input
    #[serde(rename = "dataInputCost")]
    pub data_input_cost: i32,
    /// Validation cost per one transaction output
    #[serde(rename = "outputCost")]
    pub output_cost: i32,
}

impl Parameters {
    pub fn new(height: i32, storage_fee_factor: i32, min_value_per_byte: i32, max_block_size: i32, max_block_cost: i32, block_version: i32, token_access_cost: i32, input_cost: i32, data_input_cost: i32, output_cost: i32) -> Parameters {
        Parameters {
            height,
            storage_fee_factor,
            min_value_per_byte,
            max_block_size,
            max_block_cost,
            block_version,
            token_access_cost,
            input_cost,
            data_input_cost,
            output_cost,
        }
    }
}



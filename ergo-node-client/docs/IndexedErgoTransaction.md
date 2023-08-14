# IndexedErgoTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Base16-encoded transaction id bytes | 
**inputs** | [**Vec<crate::models::ErgoTransactionInput>**](ErgoTransactionInput.md) | Transaction inputs | 
**data_inputs** | [**Vec<crate::models::ErgoTransactionDataInput>**](ErgoTransactionDataInput.md) | Transaction data inputs | 
**outputs** | [**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md) | Transaction outputs | 
**inclusion_height** | **i32** | Height of a block the transaction was included in | 
**num_confirmations** | **i32** | Number of transaction confirmations | 
**block_id** | **String** | Base16-encoded 32 byte modifier id | 
**timestamp** | **i64** | Basic timestamp definition | 
**index** | **i32** | index of the transaction in the block it was included in | 
**global_index** | **i64** | Global index of the transaction in the blockchain | 
**size** | **i32** | Size in bytes | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



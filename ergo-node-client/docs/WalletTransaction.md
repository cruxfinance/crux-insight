# WalletTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Base16-encoded transaction id bytes | [optional]
**inputs** | [**Vec<crate::models::ErgoTransactionInput>**](ErgoTransactionInput.md) | Transaction inputs | 
**data_inputs** | [**Vec<crate::models::ErgoTransactionDataInput>**](ErgoTransactionDataInput.md) | Transaction data inputs | 
**outputs** | [**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md) | Transaction outputs | 
**inclusion_height** | **i32** | Height of a block the transaction was included in | 
**num_confirmations** | **i32** | Number of transaction confirmations | 
**scans** | **Vec<i32>** | Scan identifiers the transaction relates to | 
**size** | Option<**i32**> | Size in bytes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



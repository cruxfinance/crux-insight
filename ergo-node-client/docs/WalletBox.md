# WalletBox

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#box** | [**crate::models::ErgoTransactionOutput**](ErgoTransactionOutput.md) |  | 
**confirmations_num** | Option<**i32**> | Number of confirmations, if the box is included into the blockchain | 
**address** | **String** | Encoded Ergo Address | 
**creation_transaction** | **String** | Base16-encoded 32 byte modifier id | 
**spending_transaction** | **String** | Base16-encoded 32 byte modifier id | 
**spending_height** | Option<**i32**> | The height the box was spent at | 
**inclusion_height** | **i32** | The height the transaction containing the box was included in a block at | 
**onchain** | **bool** | A flag signalling whether the box is created on main chain | 
**spent** | **bool** | A flag signalling whether the box was spent | 
**creation_out_index** | **i32** | An index of a box in the creating transaction | 
**scans** | **Vec<i32>** | Scan identifiers the box relates to | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



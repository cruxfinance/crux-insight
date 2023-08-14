# ErgoTransactionOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**box_id** | Option<**String**> | Base16-encoded transaction box id bytes. Should be 32 bytes long | [optional]
**value** | **i64** | Amount of Ergo token | 
**ergo_tree** | **String** | Base16-encoded ergo tree bytes | 
**creation_height** | **i32** | Height the output was created at | 
**assets** | Option<[**Vec<crate::models::Asset>**](Asset.md)> | Assets list in the transaction | [optional]
**additional_registers** | **::std::collections::HashMap<String, String>** | Ergo box registers | 
**transaction_id** | Option<**String**> | Base16-encoded transaction id bytes | [optional]
**index** | Option<**i32**> | Index in the transaction outputs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



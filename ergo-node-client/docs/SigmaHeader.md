# SigmaHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Base16-encoded 32 byte modifier id | [optional]
**timestamp** | **i64** | Basic timestamp definition | 
**version** | **i32** | Ergo blockchain protocol version | 
**ad_proofs_root** | **String** | Base16-encoded 32 byte digest | 
**ad_proofs_id** | Option<**String**> | Base16-encoded 32 byte modifier id | [optional]
**state_root** | [**crate::models::AvlTreeData**](AvlTreeData.md) |  | 
**transactions_root** | **String** | Base16-encoded 32 byte digest | 
**transactions_id** | Option<**String**> | Base16-encoded 32 byte modifier id | [optional]
**n_bits** | **i64** |  | 
**extension_hash** | **String** | Base16-encoded 32 byte digest | 
**extension_root** | Option<**String**> | Base16-encoded 32 byte digest | [optional]
**extension_id** | Option<**String**> | Base16-encoded 32 byte modifier id | [optional]
**height** | **i32** |  | 
**size** | Option<**i32**> |  | [optional]
**parent_id** | **String** | Base16-encoded 32 byte modifier id | 
**pow_solutions** | Option<[**crate::models::PowSolutions**](PowSolutions.md)> |  | [optional]
**votes** | **String** | Base16-encoded votes for a soft-fork and parameters | 
**miner_pk** | Option<**String**> |  | [optional]
**pow_onetime_pk** | Option<**String**> |  | [optional]
**pow_nonce** | Option<**String**> | Base16-encoded 32 byte digest | [optional]
**pow_distance** | Option<**f32**> | sigma.BigInt | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



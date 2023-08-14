# BlockHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Base16-encoded 32 byte modifier id | 
**timestamp** | **i64** | Basic timestamp definition | 
**version** | **i32** | Ergo blockchain protocol version | 
**ad_proofs_root** | **String** | Base16-encoded 32 byte digest | 
**state_root** | **String** | Base16-encoded 33 byte digest - digest with extra byte with tree height | 
**transactions_root** | **String** | Base16-encoded 32 byte digest | 
**n_bits** | **i64** |  | 
**extension_hash** | **String** | Base16-encoded 32 byte digest | 
**pow_solutions** | [**crate::models::PowSolutions**](PowSolutions.md) |  | 
**height** | **i32** |  | 
**difficulty** | **String** |  | 
**parent_id** | **String** | Base16-encoded 32 byte modifier id | 
**votes** | **String** | Base16-encoded votes for a soft-fork and parameters | 
**size** | Option<**i32**> | Size in bytes | [optional]
**extension_id** | Option<**String**> | Base16-encoded 32 byte modifier id | [optional]
**transactions_id** | Option<**String**> | Base16-encoded 32 byte modifier id | [optional]
**ad_proofs_id** | Option<**String**> | Base16-encoded 32 byte modifier id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



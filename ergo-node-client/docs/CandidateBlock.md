# CandidateBlock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**i32**> |  | [optional]
**extension_hash** | **String** | Base16-encoded 32 byte digest | 
**timestamp** | Option<**i64**> | Basic timestamp definition | [optional]
**state_root** | Option<**String**> | Base16-encoded 33 byte digest - digest with extra byte with tree height | [optional]
**n_bits** | Option<**i64**> |  | [optional]
**ad_proof_bytes** | Option<**String**> | Base16-encoded ad proofs | [optional]
**parent_id** | **String** | Base16-encoded 32 byte modifier id | 
**transactions_number** | Option<**i32**> |  | [optional]
**transactions** | Option<[**Vec<crate::models::ErgoTransaction>**](ErgoTransaction.md)> | Ergo transaction objects | [optional]
**votes** | Option<**String**> | Base16-encoded votes for a soft-fork and parameters | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



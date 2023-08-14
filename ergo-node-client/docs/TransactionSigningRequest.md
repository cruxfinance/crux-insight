# TransactionSigningRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx** | [**crate::models::UnsignedErgoTransaction**](.md) |  | 
**inputs_raw** | Option<**Vec<String>**> | Optional list of inputs to be used in serialized form | [optional]
**data_inputs_raw** | Option<**Vec<String>**> | Optional list of inputs to be used in serialized form | [optional]
**hints** | Option<[**crate::models::TransactionHintsBag**](TransactionHintsBag.md)> |  | [optional]
**secrets** | [**crate::models::TransactionSigningRequestSecrets**](TransactionSigningRequest_secrets.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



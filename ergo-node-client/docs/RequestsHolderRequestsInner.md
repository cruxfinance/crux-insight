# RequestsHolderRequestsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | Encoded Ergo Address | 
**value** | **i64** | Payment amount | 
**assets** | Option<[**Vec<crate::models::Asset>**](Asset.md)> | Assets list in the transaction | [optional]
**registers** | Option<**::std::collections::HashMap<String, String>**> | Ergo box registers | [optional]
**assets_to_burn** | [**Vec<crate::models::Asset>**](Asset.md) | Assets list to burn in the transaction | 
**erg_value** | Option<**i64**> | Optional, amount of ergs to be put into box with issued assets | [optional]
**amount** | **i64** | Supply amount | 
**name** | **String** | Assets name | 
**description** | **String** | Assets description | 
**decimals** | **i32** | Number of decimal places | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



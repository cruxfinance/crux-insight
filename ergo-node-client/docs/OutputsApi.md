# \OutputsApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_unconfirmed_transaction_output_box_by_id**](OutputsApi.md#get_unconfirmed_transaction_output_box_by_id) | **GET** /transactions/unconfirmed/outputs/byBoxId/{boxId} | Get output box from unconfirmed transactions in pool
[**get_unconfirmed_transaction_output_boxes_by_ergo_tree**](OutputsApi.md#get_unconfirmed_transaction_output_boxes_by_ergo_tree) | **POST** /transactions/unconfirmed/outputs/byErgoTree | Finds all output boxes by ErgoTree hex among unconfirmed transactions
[**get_unconfirmed_transaction_output_boxes_by_registers**](OutputsApi.md#get_unconfirmed_transaction_output_boxes_by_registers) | **POST** /transactions/unconfirmed/outputs/byRegisters | Finds all output boxes among unconfirmed transactions that contain given registers
[**get_unconfirmed_transaction_output_boxes_by_token_id**](OutputsApi.md#get_unconfirmed_transaction_output_boxes_by_token_id) | **GET** /transactions/unconfirmed/outputs/byTokenId/{tokenId} | Get output box from unconfirmed transactions in pool by tokenId



## get_unconfirmed_transaction_output_box_by_id

> crate::models::ErgoTransactionOutput get_unconfirmed_transaction_output_box_by_id(box_id)
Get output box from unconfirmed transactions in pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_id** | **String** | ID of an output box in question | [required] |

### Return type

[**crate::models::ErgoTransactionOutput**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_transaction_output_boxes_by_ergo_tree

> Vec<crate::models::ErgoTransactionOutput> get_unconfirmed_transaction_output_boxes_by_ergo_tree(body, limit, offset)
Finds all output boxes by ErgoTree hex among unconfirmed transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**limit** | Option<**i32**> | The number of items in list to return |  |[default to 50]
**offset** | Option<**i32**> | The number of items in list to skip |  |[default to 0]

### Return type

[**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_transaction_output_boxes_by_registers

> Vec<crate::models::ErgoTransactionOutput> get_unconfirmed_transaction_output_boxes_by_registers(request_body, limit, offset)
Finds all output boxes among unconfirmed transactions that contain given registers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**::std::collections::HashMap<String, String>**](String.md) |  | [required] |
**limit** | Option<**i32**> | The number of items in list to return |  |[default to 50]
**offset** | Option<**i32**> | The number of items in list to skip |  |[default to 0]

### Return type

[**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_transaction_output_boxes_by_token_id

> Vec<crate::models::ErgoTransactionOutput> get_unconfirmed_transaction_output_boxes_by_token_id(token_id)
Get output box from unconfirmed transactions in pool by tokenId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | ID of a token in question | [required] |

### Return type

[**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


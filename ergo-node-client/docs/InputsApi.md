# \InputsApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_unconfirmed_transaction_input_box_by_id**](InputsApi.md#get_unconfirmed_transaction_input_box_by_id) | **GET** /transactions/unconfirmed/inputs/byBoxId/{boxId} | Get input box from unconfirmed transactions in pool



## get_unconfirmed_transaction_input_box_by_id

> crate::models::ErgoTransactionOutput get_unconfirmed_transaction_input_box_by_id(box_id)
Get input box from unconfirmed transactions in pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_id** | **String** | ID of an input box in question | [required] |

### Return type

[**crate::models::ErgoTransactionOutput**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \UtxoApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**genesis_boxes**](UtxoApi.md#genesis_boxes) | **GET** /utxo/genesis | Get genesis boxes (boxes existed before the very first block)
[**get_box_by_id**](UtxoApi.md#get_box_by_id) | **GET** /utxo/byId/{boxId} | Get box contents for a box by a unique identifier.
[**get_box_by_id_binary**](UtxoApi.md#get_box_by_id_binary) | **GET** /utxo/byIdBinary/{boxId} | Get serialized box from UTXO pool in Base16 encoding by an identifier.
[**get_box_with_pool_by_id**](UtxoApi.md#get_box_with_pool_by_id) | **GET** /utxo/withPool/byId/{boxId} | Get box contents for a box by a unique identifier, from UTXO set and also the mempool.
[**get_box_with_pool_by_id_binary**](UtxoApi.md#get_box_with_pool_by_id_binary) | **GET** /utxo/withPool/byIdBinary/{boxId} | Get serialized box in Base16 encoding by an identifier, considering also the mempool.
[**get_boxes_binary_proof**](UtxoApi.md#get_boxes_binary_proof) | **POST** /utxo/getBoxesBinaryProof | Get serialized batch proof for given set of boxes
[**get_snapshots_info**](UtxoApi.md#get_snapshots_info) | **GET** /utxo/getSnapshotsInfo | Get information about locally stored UTXO snapshots



## genesis_boxes

> Vec<crate::models::ErgoTransactionOutput> genesis_boxes()
Get genesis boxes (boxes existed before the very first block)

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_by_id

> crate::models::ErgoTransactionOutput get_box_by_id(box_id)
Get box contents for a box by a unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_id** | **String** | ID of a wanted box | [required] |

### Return type

[**crate::models::ErgoTransactionOutput**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_by_id_binary

> crate::models::SerializedBox get_box_by_id_binary(box_id)
Get serialized box from UTXO pool in Base16 encoding by an identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_id** | **String** | ID of a wanted box | [required] |

### Return type

[**crate::models::SerializedBox**](SerializedBox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_with_pool_by_id

> crate::models::ErgoTransactionOutput get_box_with_pool_by_id(box_id)
Get box contents for a box by a unique identifier, from UTXO set and also the mempool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_id** | **String** | ID of a box to obtain | [required] |

### Return type

[**crate::models::ErgoTransactionOutput**](ErgoTransactionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_with_pool_by_id_binary

> crate::models::SerializedBox get_box_with_pool_by_id_binary(box_id)
Get serialized box in Base16 encoding by an identifier, considering also the mempool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_id** | **String** | ID of a wanted box | [required] |

### Return type

[**crate::models::SerializedBox**](SerializedBox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_boxes_binary_proof

> String get_boxes_binary_proof(request_body)
Get serialized batch proof for given set of boxes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshots_info

> get_snapshots_info()
Get information about locally stored UTXO snapshots

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \BlockchainApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_address_balance_total**](BlockchainApi.md#get_address_balance_total) | **POST** /blockchain/balance | Retrieve confirmed and unconfirmed balance of an address
[**get_box_by_id**](BlockchainApi.md#get_box_by_id) | **GET** /blockchain/box/byId/{boxId} | Retrieve a box by its id
[**get_box_by_index**](BlockchainApi.md#get_box_by_index) | **GET** /blockchain/box/byIndex/{boxIndex} | Retrieve a box by global index number
[**get_box_range**](BlockchainApi.md#get_box_range) | **GET** /blockchain/box/range | Get a range of box ids
[**get_boxes_by_address**](BlockchainApi.md#get_boxes_by_address) | **POST** /blockchain/box/byAddress | Retrieve boxes by their associated address
[**get_boxes_by_address_unspent**](BlockchainApi.md#get_boxes_by_address_unspent) | **POST** /blockchain/box/unspent/byAddress | Retrieve unspent boxes by their associated address
[**get_boxes_by_ergo_tree**](BlockchainApi.md#get_boxes_by_ergo_tree) | **POST** /blockchain/box/byErgoTree | Retrieve boxes by their associated ergotree
[**get_boxes_by_ergo_tree_unspent**](BlockchainApi.md#get_boxes_by_ergo_tree_unspent) | **POST** /blockchain/box/unspent/byErgoTree | Retrieve unspent boxes by their associated ergotree
[**get_indexed_height**](BlockchainApi.md#get_indexed_height) | **GET** /blockchain/indexedHeight | Get current block height the indexer is at
[**get_token_by_id**](BlockchainApi.md#get_token_by_id) | **GET** /blockchain/token/byId/{tokenId} | Retrieve minting information about a token
[**get_tx_by_id**](BlockchainApi.md#get_tx_by_id) | **GET** /blockchain/transaction/byId/{txId} | Retrieve a transaction by its id
[**get_tx_by_index**](BlockchainApi.md#get_tx_by_index) | **GET** /blockchain/transaction/byIndex/{txIndex} | Retrieve a transaction by global index number
[**get_tx_range**](BlockchainApi.md#get_tx_range) | **GET** /blockchain/transaction/range | Get a range of transaction ids
[**get_txs_by_address**](BlockchainApi.md#get_txs_by_address) | **POST** /blockchain/transaction/byAddress | Retrieve transactions by their associated address



## get_address_balance_total

> crate::models::GetAddressBalanceTotal200Response get_address_balance_total(body)
Retrieve confirmed and unconfirmed balance of an address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

[**crate::models::GetAddressBalanceTotal200Response**](getAddressBalanceTotal_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_by_id

> crate::models::IndexedErgoBox get_box_by_id(box_id)
Retrieve a box by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_id** | **String** | id of the wanted box | [required] |

### Return type

[**crate::models::IndexedErgoBox**](IndexedErgoBox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_by_index

> crate::models::IndexedErgoBox get_box_by_index(box_index)
Retrieve a box by global index number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_index** | **f32** | index of the wanted box | [required] |

### Return type

[**crate::models::IndexedErgoBox**](IndexedErgoBox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_range

> Vec<String> get_box_range(offset, limit)
Get a range of box ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | amount of elements to skip from the start |  |[default to 0]
**limit** | Option<**i32**> | amount of elements to retrieve |  |[default to 5]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_boxes_by_address

> crate::models::GetBoxesByAddress200Response get_boxes_by_address(body, offset, limit)
Retrieve boxes by their associated address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**offset** | Option<**i32**> | amount of elements to skip from the start |  |[default to 0]
**limit** | Option<**i32**> | amount of elements to retrieve |  |[default to 5]

### Return type

[**crate::models::GetBoxesByAddress200Response**](getBoxesByAddress_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_boxes_by_address_unspent

> Vec<crate::models::IndexedErgoBox> get_boxes_by_address_unspent(body, offset, limit, sort_direction)
Retrieve unspent boxes by their associated address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**offset** | Option<**i32**> | amount of elements to skip from the start |  |[default to 0]
**limit** | Option<**i32**> | amount of elements to retrieve |  |[default to 5]
**sort_direction** | Option<**String**> | desc = new boxes first ; asc = old boxes first |  |[default to desc]

### Return type

[**Vec<crate::models::IndexedErgoBox>**](IndexedErgoBox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_boxes_by_ergo_tree

> crate::models::GetBoxesByAddress200Response get_boxes_by_ergo_tree(body, offset, limit)
Retrieve boxes by their associated ergotree

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**offset** | Option<**i32**> | amount of elements to skip from the start |  |[default to 0]
**limit** | Option<**i32**> | amount of elements to retrieve |  |[default to 5]

### Return type

[**crate::models::GetBoxesByAddress200Response**](getBoxesByAddress_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_boxes_by_ergo_tree_unspent

> crate::models::GetBoxesByAddress200Response get_boxes_by_ergo_tree_unspent(body, offset, limit, sort_direction)
Retrieve unspent boxes by their associated ergotree

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**offset** | Option<**i32**> | amount of elements to skip from the start |  |[default to 0]
**limit** | Option<**i32**> | amount of elements to retrieve |  |[default to 5]
**sort_direction** | Option<**String**> | desc = new boxes first ; asc = old boxes first |  |[default to desc]

### Return type

[**crate::models::GetBoxesByAddress200Response**](getBoxesByAddress_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_indexed_height

> crate::models::GetIndexedHeight200Response get_indexed_height()
Get current block height the indexer is at

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetIndexedHeight200Response**](getIndexedHeight_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_by_id

> crate::models::IndexedToken get_token_by_id(token_id)
Retrieve minting information about a token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | id of the wanted token | [required] |

### Return type

[**crate::models::IndexedToken**](IndexedToken.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx_by_id

> crate::models::IndexedErgoTransaction get_tx_by_id(tx_id)
Retrieve a transaction by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | id of the wanted transaction | [required] |

### Return type

[**crate::models::IndexedErgoTransaction**](IndexedErgoTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx_by_index

> crate::models::IndexedErgoTransaction get_tx_by_index(tx_index)
Retrieve a transaction by global index number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_index** | **f32** | index of the wanted transaction | [required] |

### Return type

[**crate::models::IndexedErgoTransaction**](IndexedErgoTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx_range

> Vec<String> get_tx_range(offset, limit)
Get a range of transaction ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | amount of elements to skip from the start |  |[default to 0]
**limit** | Option<**i32**> | amount of elements to retrieve |  |[default to 5]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs_by_address

> crate::models::GetTxsByAddress200Response get_txs_by_address(body, offset, limit)
Retrieve transactions by their associated address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**offset** | Option<**i32**> | amount of elements to skip from the start |  |[default to 0]
**limit** | Option<**i32**> | amount of elements to retrieve |  |[default to 5]

### Return type

[**crate::models::GetTxsByAddress200Response**](getTxsByAddress_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


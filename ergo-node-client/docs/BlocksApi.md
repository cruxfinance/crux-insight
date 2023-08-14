# \BlocksApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_header_by_id**](BlocksApi.md#get_block_header_by_id) | **GET** /blocks/{headerId}/header | Get the block header info by a given signature
[**get_block_transactions_by_id**](BlocksApi.md#get_block_transactions_by_id) | **GET** /blocks/{headerId}/transactions | Get the block transactions info by a given signature
[**get_chain_slice**](BlocksApi.md#get_chain_slice) | **GET** /blocks/chainSlice | Get headers in a specified range
[**get_full_block_at**](BlocksApi.md#get_full_block_at) | **GET** /blocks/at/{blockHeight} | Get the header ids at a given height
[**get_full_block_by_id**](BlocksApi.md#get_full_block_by_id) | **GET** /blocks/{headerId} | Get the full block info by a given signature
[**get_header_ids**](BlocksApi.md#get_header_ids) | **GET** /blocks | Get the Array of header ids
[**get_last_headers**](BlocksApi.md#get_last_headers) | **GET** /blocks/lastHeaders/{count} | Get the last headers objects
[**get_modifier_by_id**](BlocksApi.md#get_modifier_by_id) | **GET** /blocks/modifier/{modifierId} | Get the persistent modifier by its id
[**get_proof_for_tx**](BlocksApi.md#get_proof_for_tx) | **GET** /blocks/{headerId}/proofFor/{txId} | Get Merkle proof for transaction
[**send_mined_block**](BlocksApi.md#send_mined_block) | **POST** /blocks | Send a mined block



## get_block_header_by_id

> crate::models::BlockHeader get_block_header_by_id(header_id)
Get the block header info by a given signature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**header_id** | **String** | ID of a wanted block header | [required] |

### Return type

[**crate::models::BlockHeader**](BlockHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_transactions_by_id

> crate::models::BlockTransactions get_block_transactions_by_id(header_id)
Get the block transactions info by a given signature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**header_id** | **String** | ID of a wanted block transactions | [required] |

### Return type

[**crate::models::BlockTransactions**](BlockTransactions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chain_slice

> Vec<crate::models::BlockHeader> get_chain_slice(from_height, to_height)
Get headers in a specified range

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_height** | Option<**i32**> | Min header height |  |[default to 0]
**to_height** | Option<**i32**> | Max header height (best header height by default) |  |[default to -1]

### Return type

[**Vec<crate::models::BlockHeader>**](BlockHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_full_block_at

> Vec<String> get_full_block_at(block_height)
Get the header ids at a given height

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_height** | **i32** | Height of a wanted block | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_full_block_by_id

> crate::models::FullBlock get_full_block_by_id(header_id)
Get the full block info by a given signature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**header_id** | **String** | ID of a wanted block | [required] |

### Return type

[**crate::models::FullBlock**](FullBlock.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_header_ids

> Vec<String> get_header_ids(limit, offset)
Get the Array of header ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items in list to return |  |[default to 50]
**offset** | Option<**i32**> | The number of items in list to skip |  |[default to 0]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_last_headers

> Vec<crate::models::BlockHeader> get_last_headers(count)
Get the last headers objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | **f32** | count of a wanted block headers | [required] |

### Return type

[**Vec<crate::models::BlockHeader>**](BlockHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_modifier_by_id

> get_modifier_by_id(modifier_id)
Get the persistent modifier by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modifier_id** | **String** | ID of a wanted modifier | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_proof_for_tx

> crate::models::MerkleProof get_proof_for_tx(header_id, tx_id)
Get Merkle proof for transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**header_id** | **String** | ID of a wanted block transactions | [required] |
**tx_id** | **String** | ID of a wanted transaction | [required] |

### Return type

[**crate::models::MerkleProof**](MerkleProof.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_mined_block

> send_mined_block(full_block)
Send a mined block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_block** | [**FullBlock**](FullBlock.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


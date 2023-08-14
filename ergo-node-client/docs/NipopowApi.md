# \NipopowApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_popow_header_by_height**](NipopowApi.md#get_popow_header_by_height) | **GET** /nipopow/popowHeaderByHeight/{height} | Construct PoPow header for best header at given height
[**get_popow_header_by_id**](NipopowApi.md#get_popow_header_by_id) | **GET** /nipopow/popowHeaderById/{headerId} | Construct PoPow header according to given header id
[**get_popow_proof**](NipopowApi.md#get_popow_proof) | **GET** /nipopow/proof/{minChainLength}/{suffixLength} | Construct PoPoW proof for given min superchain length and suffix length
[**get_popow_proof_by_header_id**](NipopowApi.md#get_popow_proof_by_header_id) | **GET** /nipopow/proof/{minChainLength}/{suffixLength}/{headerId} | Construct PoPoW proof for given min superchain length, suffix length and header ID



## get_popow_header_by_height

> crate::models::PopowHeader get_popow_header_by_height(height)
Construct PoPow header for best header at given height

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | **i32** | Height of a wanted header | [required] |

### Return type

[**crate::models::PopowHeader**](PopowHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_popow_header_by_id

> crate::models::PopowHeader get_popow_header_by_id(header_id)
Construct PoPow header according to given header id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**header_id** | **String** | ID of wanted header | [required] |

### Return type

[**crate::models::PopowHeader**](PopowHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_popow_proof

> crate::models::NipopowProof get_popow_proof(min_chain_length, suffix_length)
Construct PoPoW proof for given min superchain length and suffix length

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_chain_length** | **f32** | Minimal superchain length | [required] |
**suffix_length** | **f32** | Suffix length | [required] |

### Return type

[**crate::models::NipopowProof**](NipopowProof.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_popow_proof_by_header_id

> crate::models::NipopowProof get_popow_proof_by_header_id(min_chain_length, suffix_length, header_id)
Construct PoPoW proof for given min superchain length, suffix length and header ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_chain_length** | **f32** | Minimal superchain length | [required] |
**suffix_length** | **f32** | Suffix length | [required] |
**header_id** | **String** | ID of wanted header | [required] |

### Return type

[**crate::models::NipopowProof**](NipopowProof.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


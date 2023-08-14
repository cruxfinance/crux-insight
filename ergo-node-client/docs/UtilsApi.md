# \UtilsApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**address_to_raw**](UtilsApi.md#address_to_raw) | **GET** /utils/addressToRaw/{address} | Convert Pay-To-Public-Key Address to raw representation (hex-encoded serialized curve point)
[**check_address_validity**](UtilsApi.md#check_address_validity) | **POST** /utils/address | Checks address validity
[**check_address_validity_with_get**](UtilsApi.md#check_address_validity_with_get) | **GET** /utils/address/{address} | Check address validity (prefer POST request as addresses can be too big)
[**ergo_tree_to_address**](UtilsApi.md#ergo_tree_to_address) | **POST** /utils/ergoTreeToAddress | Generate Ergo address from hex-encoded ErgoTree
[**ergo_tree_to_address_with_get**](UtilsApi.md#ergo_tree_to_address_with_get) | **GET** /utils/ergoTreeToAddress/{ergoTreeHex} | Generate Ergo address from hex-encoded ErgoTree (prefer POST request as ErgoTree can be too big)
[**get_random_seed**](UtilsApi.md#get_random_seed) | **GET** /utils/seed | Get random seed of 32 bytes
[**get_random_seed_with_length**](UtilsApi.md#get_random_seed_with_length) | **GET** /utils/seed/{length} | Generate random seed of specified length in bytes
[**hash_blake2b**](UtilsApi.md#hash_blake2b) | **POST** /utils/hash/blake2b | Return Blake2b hash of specified message
[**raw_to_address**](UtilsApi.md#raw_to_address) | **GET** /utils/rawToAddress/{pubkeyHex} | Generate Pay-To-Public-Key address from hex-encoded raw pubkey (secp256k1 serialized point)



## address_to_raw

> String address_to_raw(address)
Convert Pay-To-Public-Key Address to raw representation (hex-encoded serialized curve point)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address to extract public key from | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_address_validity

> crate::models::AddressValidity check_address_validity(body)
Checks address validity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | address to check | [required] |

### Return type

[**crate::models::AddressValidity**](AddressValidity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_address_validity_with_get

> crate::models::AddressValidity check_address_validity_with_get(address)
Check address validity (prefer POST request as addresses can be too big)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address to check | [required] |

### Return type

[**crate::models::AddressValidity**](AddressValidity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ergo_tree_to_address

> String ergo_tree_to_address(body)
Generate Ergo address from hex-encoded ErgoTree

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | ErgoTree hex to derive an address from | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ergo_tree_to_address_with_get

> String ergo_tree_to_address_with_get(ergo_tree_hex)
Generate Ergo address from hex-encoded ErgoTree (prefer POST request as ErgoTree can be too big)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ergo_tree_hex** | **String** | ErgoTree to derive an address from | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_random_seed

> String get_random_seed()
Get random seed of 32 bytes

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_random_seed_with_length

> String get_random_seed_with_length(length)
Generate random seed of specified length in bytes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**length** | **String** | seed length in bytes | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hash_blake2b

> String hash_blake2b(body)
Return Blake2b hash of specified message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## raw_to_address

> String raw_to_address(pubkey_hex)
Generate Pay-To-Public-Key address from hex-encoded raw pubkey (secp256k1 serialized point)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey_hex** | **String** | public key to get address from | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \ScriptApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**address_to_bytes**](ScriptApi.md#address_to_bytes) | **GET** /script/addressToBytes/{address} | Convert an address to hex-encoded Sigma byte array constant which contains script bytes
[**address_to_tree**](ScriptApi.md#address_to_tree) | **GET** /script/addressToTree/{address} | Convert an address to hex-encoded serialized ErgoTree (script)
[**execute_with_context**](ScriptApi.md#execute_with_context) | **POST** /script/executeWithContext | Execute script with context
[**script_p2_s_address**](ScriptApi.md#script_p2_s_address) | **POST** /script/p2sAddress | Create P2SAddress from Sigma source
[**script_p2_sh_address**](ScriptApi.md#script_p2_sh_address) | **POST** /script/p2shAddress | Create P2SHAddress from Sigma source



## address_to_bytes

> crate::models::ScriptBytes address_to_bytes(address)
Convert an address to hex-encoded Sigma byte array constant which contains script bytes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address to get a script from | [required] |

### Return type

[**crate::models::ScriptBytes**](ScriptBytes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## address_to_tree

> crate::models::ErgoTreeObject address_to_tree(address)
Convert an address to hex-encoded serialized ErgoTree (script)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address to get a script from | [required] |

### Return type

[**crate::models::ErgoTreeObject**](ErgoTreeObject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_with_context

> crate::models::CryptoResult execute_with_context(execute_script)
Execute script with context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execute_script** | [**ExecuteScript**](ExecuteScript.md) |  | [required] |

### Return type

[**crate::models::CryptoResult**](CryptoResult.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## script_p2_s_address

> crate::models::AddressHolder script_p2_s_address(source_holder)
Create P2SAddress from Sigma source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_holder** | [**SourceHolder**](SourceHolder.md) |  | [required] |

### Return type

[**crate::models::AddressHolder**](AddressHolder.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## script_p2_sh_address

> crate::models::AddressHolder script_p2_sh_address(source_holder)
Create P2SHAddress from Sigma source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_holder** | [**SourceHolder**](SourceHolder.md) |  | [required] |

### Return type

[**crate::models::AddressHolder**](AddressHolder.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


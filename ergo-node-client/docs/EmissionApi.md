# \EmissionApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**emission_at**](EmissionApi.md#emission_at) | **GET** /emission/at/{blockHeight} | Get emission data for a given height
[**emission_scripts**](EmissionApi.md#emission_scripts) | **GET** /emission/scripts | Print emission-related scripts



## emission_at

> crate::models::EmissionInfo emission_at(block_height)
Get emission data for a given height

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_height** | **i32** | Height to get emission data for | [required] |

### Return type

[**crate::models::EmissionInfo**](EmissionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emission_scripts

> crate::models::EmissionScripts emission_scripts()
Print emission-related scripts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EmissionScripts**](EmissionScripts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

